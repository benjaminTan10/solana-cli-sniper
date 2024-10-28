use std::str::FromStr;
use std::sync::Arc;

use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::VersionedTransaction;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;

use crate::env::{load_config, SettingsConfig};
use crate::input::{amount_input, mint_input, percentage_input};
use crate::instruction::instruction::compute_ixs;
use crate::liquidity::utils::tip_account;
use crate::raydium_amm::swap::swapper::auth_keypair;

use super::dao_burned_interface::InitializeIxData;
use super::inx_builder::{create_buy_instruction, create_sell_instruction};

#[derive(PartialEq, Debug)]
pub enum DAOSTrade {
    Buy,
    Sell,
}

pub const DAOS_PROGRAM: Pubkey = pubkey!("5jnapfrAN47UYkLkEf7HnprPPBCQLvkYWGZDeKkaP5hv");
pub const DAOS_BURNED_PROGRAM: Pubkey = pubkey!("4FqThZWv3QKWkSyXCDmATpWkpEiCHq5yhkdGWpSEDAZM");
pub const SIMPLE_PROGRAM: Pubkey = pubkey!("ETK5PUmiqVDRsd1TPFqCu84bsrLNG4YySZND96PEjW97");

pub async fn interact_daos_token(direction: DAOSTrade) -> eyre::Result<()> {
    let args = load_config().await?;

    let wallet = Keypair::from_base58_string(&args.engine.payer_keypair);

    let dao_mint = mint_input("Input DAO Mint:").await;

    let (state, _) =
        Pubkey::find_program_address(&[b"state".as_ref(), dao_mint.as_ref()], &DAOS_PROGRAM);

    let mut amount;
    if direction == DAOSTrade::Buy {
        amount = amount_input("Input SOL amount:").await;
    } else {
        amount = percentage_input().await as u64;
    }

    let rpc_client = RpcClient::new(args.clone().network.rpc_url);

    let token = rpc_client.get_account(&dao_mint).await?;
    // let mint = Mint::unpack(&token).unwrap();
    let dao_mint_program = token.owner;

    let _ = daosfun_sender(
        rpc_client,
        args,
        wallet,
        dao_mint,
        dao_mint_program,
        amount,
        direction,
    )
    .await?;

    Ok(())
}

pub async fn daosfun_sender(
    rpc_client: RpcClient,
    args: SettingsConfig,
    wallet: Keypair,
    dao_mint: Pubkey,
    dao_mint_program: Pubkey,
    amount: u64,
    direction: DAOSTrade,
) -> eyre::Result<()> {
    let compute = compute_ixs(sol_to_lamports(args.trading.priority_fee), 80000)?;

    let account = create_associated_token_account_idempotent(
        &wallet.pubkey(),
        &wallet.pubkey(),
        &dao_mint,
        &dao_mint_program,
    );

    let mut swap_instructions = vec![account];
    swap_instructions.extend(compute);
    if direction == DAOSTrade::Buy {
        let buy_instruction = create_buy_instruction(
            &DAOS_PROGRAM,
            &wallet.pubkey(),
            &dao_mint,
            &dao_mint_program,
            amount,
            0,
        )?;
        swap_instructions.push(buy_instruction);
    } else {
        let sell_instruction = create_sell_instruction(
            &DAOS_PROGRAM,
            &wallet.pubkey(),
            &dao_mint,
            &dao_mint_program,
            amount,
            0,
        )
        .await?;
        swap_instructions.push(sell_instruction);
    }

    let config = CommitmentLevel::Processed;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &wallet.pubkey(),
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
        let mut searcher_client =
            get_searcher_client(&args.network.block_engine_url, &Arc::new(auth_keypair())).await?;

        let tip_ix = transfer(
            &wallet.pubkey(),
            &tip_account(),
            sol_to_lamports(args.trading.bundle_tip),
        );
        swap_instructions.push(tip_ix);

        let message = match solana_program::message::v0::Message::try_compile(
            &wallet.pubkey(),
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

        let bundle_txn = vec![transaction];

        let mut bundle_results_subscription = searcher_client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        match send_bundle_with_confirmation(
            &bundle_txn,
            &Arc::new(rpc_client),
            &mut searcher_client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                error!("Error: {}", e);
            }
        };

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

            rpc_client
                .confirm_transaction_with_spinner(
                    &result,
                    &latest_blockhash,
                    CommitmentConfig::confirmed(),
                )
                .await?;

            info!("Transaction Sent {:?}", result);
        }
    }

    Ok(())
}
