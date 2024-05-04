use std::str::FromStr;

use demand::Confirm;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::{
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};

use crate::{
    env::minter::load_minter_settings,
    instruction::instruction::{
        get_keys_for_market, load_amm_keys, withdraw, AmmKeys, MarketPubkeys,
    },
    liquidity::pool_ixs::AMM_PROGRAM,
    rpc::HTTP_CLIENT,
    user_inputs::tokens::token_env,
};

pub async fn remove_liquidity() -> eyre::Result<()> {
    let pool_id = token_env("Pool ID: ").await;
    let data = load_minter_settings().await?;
    let msg = match rem_liq().await {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };
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
    let amm_keys = load_amm_keys(&client, &AMM_PROGRAM, &amm_pool_id).await?;
    // load market keys
    let token_accounts = client
        .get_token_accounts_by_owner(
            &wallet.pubkey(),
            TokenAccountsFilter::Mint(amm_keys.amm_lp_mint),
        )
        .await?;

    let mut withdraw_lp_amount = 0;
    for account in token_accounts {
        withdraw_lp_amount = account.account.lamports;
    }

    let market_keys =
        get_keys_for_market(&client, &amm_keys.market_program, &amm_keys.market).await?;

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
        &spl_associated_token_account::get_associated_token_address(
            &wallet.pubkey(),
            &amm_keys.amm_pc_mint,
        ),
        &spl_associated_token_account::get_associated_token_address(
            &wallet.pubkey(),
            &amm_keys.amm_lp_mint,
        ),
        withdraw_lp_amount,
    )?;

    // send transaction
    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &wallet.pubkey(),
            &[build_withdraw_instruction],
            &[],
            client.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let tx = VersionedTransaction::try_new(versioned_msg, &[&wallet])?;

    let signature = client.send_transaction(&tx).await?;

    println!("Withdraw transaction signature: {:?}", signature);

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

    Ok(!confirm) // Flip the boolean value because we swapped the labels
}
