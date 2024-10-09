use std::sync::Arc;
use std::time::Duration;

use crate::app::config_init::get_config;
use crate::env::utils::read_keys;
use crate::env::SettingsConfig;
use crate::input::gas_input;
use crate::liquidity::utils::tip_account;
use crate::pumpfun::pump_interface::builder::{
    generate_pump_buy_ix, generate_pump_sell_ix, PumpFunDirection,
};
use crate::raydium_amm::swap::swapper::auth_keypair;

use borsh::BorshDeserialize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_instruction::transfer;
use solana_sdk::{
    commitment_config::CommitmentLevel,
    signature::Keypair,
    signer::Signer,
    transaction::{Transaction, VersionedTransaction},
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use tokio::time::sleep;

use super::pump_interface::accounts::BondingCurve;
use super::pump_interface::builder::{calculate_sell_price, get_bonding_curve, PUMP_PROGRAM};

#[async_recursion::async_recursion]
pub async fn pump_swap(
    wallet: &Arc<Keypair>,
    args: SettingsConfig,
    direction: PumpFunDirection,
    token_address: Pubkey,
    amount: u64,
) -> eyre::Result<()> {
    let rpc_client = Arc::new(RpcClient::new(args.network.rpc_url));

    let mut bundle_tip = 0;
    if args.engine.use_bundles {
        bundle_tip = gas_input("Bundle Priority Tip: ").await;
    }

    let user_source_owner = wallet.pubkey();

    let mut searcher_client =
        get_searcher_client(&args.network.block_engine_url, &Arc::new(auth_keypair())).await?;

    let tip_account = tip_account();

    let mut swap_instructions = vec![];

    let create_account = create_associated_token_account_idempotent(
        &wallet.pubkey(),
        &wallet.pubkey(),
        &token_address,
        &spl_token::id(),
    );

    swap_instructions.push(create_account);

    if direction == PumpFunDirection::Buy {
        let buy_ix =
            generate_pump_buy_ix(rpc_client.clone(), token_address, amount, wallet.clone())
                .await
                .unwrap();
        swap_instructions.extend(buy_ix);
    } else {
        let sell_ix = generate_pump_sell_ix(token_address, amount, wallet.clone())
            .await
            .unwrap();
        swap_instructions.extend(sell_ix);
    }

    let config = CommitmentLevel::Finalized;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &swap_instructions,
        &[],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let transaction = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    if args.engine.use_bundles {
        info!("Building Bundle");

        let tip_txn = VersionedTransaction::from(Transaction::new_signed_with_payer(
            &[transfer(&wallet.pubkey(), &tip_account, bundle_tip)],
            Some(&wallet.pubkey()),
            &[&wallet],
            rpc_client.get_latest_blockhash().await.unwrap(),
        ));

        let bundle_txn = vec![transaction, tip_txn];

        let mut bundle_results_subscription = searcher_client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        match send_bundle_with_confirmation(
            &bundle_txn,
            &rpc_client,
            &mut searcher_client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: {}", e);
            }
        }

        std::mem::drop(bundle_results_subscription);
    } else {
        info!("Sending Transaction");
        let config = RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        };

        if args.trading.spam {
            let mut counter = 0;
            while counter < args.trading.spam_count {
                let result = match rpc_client
                    .send_transaction_with_config(&transaction, config)
                    .await
                {
                    Ok(x) => x,
                    Err(e) => {
                        error!("Error: {:?}", e);
                        return Ok(());
                    }
                };

                info!("Transaction Sent {:?}", result);
                counter += 1;
            }
        } else {
            let result = match rpc_client
                .send_transaction_with_config(&transaction, config)
                .await
            {
                Ok(x) => x,
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            };

            info!("Transaction Sent {:?}", result);
        }
    }

    if direction == PumpFunDirection::Buy {
        pump_tracker(amount, token_address).await?;
    }
    let _ = read_keys();

    Ok(())
}

pub async fn pump_tracker(init_buy: u64, base_mint: Pubkey) -> eyre::Result<()> {
    let config = get_config().await?;
    let wallet = Arc::new(Keypair::from_base58_string(&config.engine.payer_keypair));
    let rpc_client = Arc::new(RpcClient::new(config.clone().network.rpc_url));
    loop {
        sleep(Duration::from_secs(5)).await;
        let token_account = get_associated_token_address(&wallet.pubkey(), &base_mint);

        let tokens_amount = rpc_client
            .get_token_account_balance(&token_account)
            .await?
            .amount
            .parse::<u128>()?;
        let bonding_curve_pda = get_bonding_curve(base_mint, &PUMP_PROGRAM);
        let account_data = rpc_client.get_account_data(&bonding_curve_pda).await?;

        let sliced_data: &mut &[u8] = &mut account_data.as_slice();

        let reserves = BondingCurve::deserialize_reader(sliced_data)?;

        let reserves = (
            reserves.real_token_reserves as u128,
            reserves.virtual_sol_reserves as u128,
            reserves.real_sol_reserves as u128,
        );

        let price = calculate_sell_price(tokens_amount, reserves);

        println!("{price:#?}");

        let profit_percentage = (price.0 as u64 - init_buy) / init_buy * 100;

        if profit_percentage >= config.trading.profit_threshold_percentage as u64 {
            match pump_swap(
                &wallet,
                config.clone(),
                PumpFunDirection::Sell,
                base_mint,
                tokens_amount as u64,
            )
            .await
            {
                Ok(s) => s,
                Err(e) => {
                    log::error!("Error: {}", e);
                    return Ok(());
                }
            };
        } else if profit_percentage <= config.trading.loss_threshold_percentage as u64 {
            match pump_swap(
                &wallet,
                config.clone(),
                PumpFunDirection::Sell,
                base_mint,
                tokens_amount as u64,
            )
            .await
            {
                Ok(s) => s,
                Err(e) => {
                    log::error!("Error: {}", e);
                    return Ok(());
                }
            };
        }
    }
}
