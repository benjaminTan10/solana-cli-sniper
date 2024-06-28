use std::{str::FromStr, sync::Arc};

use demand::Confirm;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_client::{rpc_config::RpcSendTransactionConfig, rpc_request::TokenAccountsFilter};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction::create_account_with_seed,
    transaction::VersionedTransaction,
};
use spl_token::instruction::initialize_account;

use crate::{
    env::{load_settings, minter::load_minter_settings},
    instruction::instruction::{
        get_keys_for_market, load_amm_keys, withdraw, AmmKeys, MarketPubkeys,
    },
    liquidity::{
        pool_ixs::{generate_pubkey, AMM_PROGRAM},
        utils::{tip_account, tip_txn},
    },
    raydium::{
        pool_searcher::amm_keys::pool_keys_fetcher,
        swap::{instructions::SOLC_MINT, swapper::auth_keypair},
    },
    rpc::HTTP_CLIENT,
    user_inputs::tokens::token_env,
};

pub async fn remove_liquidity() -> eyre::Result<()> {
    let data = load_minter_settings().await?;
    let msg = match rem_liq().await {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };
    let engine = load_settings().await?;
    let mut pool_id = Pubkey::from_str(&data.pool_id)?;
    if data.pool_id.is_empty() {
        pool_id = token_env("Pool ID: ").await;

        return Ok(());
    }
    if msg {
        return Ok(());
    }
    let client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let wallet = Keypair::from_base58_string(&data.deployer_key);
    let amm_pool_id = pool_id;
    // config params
    // let slippage_bps = 50u64; // 0.5%

    // let withdraw_lp_amount = 10000_000000;

    // load amm keys
    let amm_keys = match load_amm_keys(&client, &AMM_PROGRAM, &amm_pool_id).await {
        Ok(amm_keys) => amm_keys,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    println!("Associated Token Address: {:?}", amm_keys.amm_lp_mint);

    let token_accounts = client
        .get_token_accounts_by_owner(
            &wallet.pubkey(),
            TokenAccountsFilter::Mint(amm_keys.amm_lp_mint),
        )
        .await?;

    let mut withdraw_lp_amount = 0;
    for account in token_accounts {
        let balance = client
            .get_token_account_balance(&Pubkey::from_str(&account.pubkey).unwrap())
            .await?;

        withdraw_lp_amount = balance.amount.parse::<u64>().unwrap();
    }

    println!("Token Amount: {}", withdraw_lp_amount);

    // for account in token_accounts {
    //     withdraw_lp_amount = account.account.lamports;
    // }

    let market_keys =
        get_keys_for_market(&client, &amm_keys.market_program, &amm_keys.market).await?;

    let mut remove_pool_inx = vec![];

    let (pubkey, seed) = generate_pubkey(wallet.pubkey()).await?;

    let inx = create_account_with_seed(
        &wallet.pubkey(),
        &pubkey,
        &wallet.pubkey(),
        &seed,
        2039280,
        165,
        &spl_token::id(),
    );

    let init = initialize_account(&spl_token::id(), &pubkey, &SOLC_MINT, &wallet.pubkey())?;

    // build withdraw instruction
    let build_withdraw_instruction = withdraw_ix(
        &AMM_PROGRAM,
        &amm_keys,
        &market_keys,
        &wallet.pubkey(),
        &spl_associated_token_account::get_associated_token_address(
            &wallet.pubkey(),
            &amm_keys.amm_coin_mint,
        ),
        &pubkey,
        &spl_associated_token_account::get_associated_token_address(
            &wallet.pubkey(),
            &amm_keys.amm_lp_mint,
        ),
        withdraw_lp_amount,
    )?;

    let tip_txn = tip_txn(wallet.pubkey(), tip_account(), sol_to_lamports(0.001));

    // remove_pool_inx.push(unit_limit);
    // remove_pool_inx.push(compute_price);
    remove_pool_inx.push(inx);
    remove_pool_inx.push(init);
    remove_pool_inx.push(build_withdraw_instruction);
    remove_pool_inx.push(tip_txn);

    // send transaction
    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &wallet.pubkey(),
            &remove_pool_inx,
            &[],
            client.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let tx = VersionedTransaction::try_new(versioned_msg, &[&wallet])?;

    let mut searcher_client =
        get_searcher_client(&engine.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = searcher_client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let bundle_results = match send_bundle_with_confirmation(
        &[tx],
        &client,
        &mut searcher_client,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(bundle_results) => bundle_results,
        Err(e) => {
            eprintln!("Error sending bundle: {}", e);
        }
    };

    Ok(())
}

pub fn withdraw_ix(
    amm_program: &Pubkey,
    amm_keys: &AmmKeys,
    market_keys: &MarketPubkeys,
    user_owner: &Pubkey,
    user_coin: &Pubkey,
    user_pc: &Pubkey,
    user_lp: &Pubkey,
    withdraw_lp_amount: u64,
) -> eyre::Result<Instruction> {
    let withdraw_instruction = withdraw(
        &amm_program,
        &amm_keys.amm_pool,
        &amm_keys.amm_authority,
        &amm_keys.amm_open_order,
        &amm_keys.amm_target,
        &amm_keys.amm_lp_mint,
        &amm_keys.amm_coin_vault,
        &amm_keys.amm_pc_vault,
        &amm_keys.market_program,
        &amm_keys.market,
        &market_keys.coin_vault,
        &market_keys.pc_vault,
        &market_keys.vault_signer_key,
        user_lp,
        user_coin,
        user_pc,
        user_owner,
        &market_keys.event_q,
        &market_keys.bids,
        &market_keys.asks,
        None,
        withdraw_lp_amount,
    )?;
    Ok(withdraw_instruction)
}

pub async fn rem_liq() -> Result<bool, Box<dyn std::error::Error>> {
    let confirm = Confirm::new("Remove Liquidity")
        .description("Entire liquidity will be removed from the pool. Are you sure?")
        .affirmative("No")
        .negative("Yes")
        .selected(false)
        .run()
        .unwrap();

    Ok(confirm)
}
