use std::str::FromStr;

use crate::{
    app::theme,
    env::minter::PoolDataSettings,
    instruction::instruction::{get_amm_pda_keys, initialize_amm_pool, AmmKeys, SOL_MINT},
    rpc::HTTP_CLIENT,
};
use demand::Input;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{
    instruction::Instruction,
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey,
};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

pub const AMM_PROGRAM: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

pub async fn pool_ixs(pool_data: PoolDataSettings) -> eyre::Result<(Instruction, Pubkey, AmmKeys)> {
    let amm_program = Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")?;
    let market_program = Pubkey::from_str("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX")?;
    let market = Pubkey::from_str(&pool_data.market_id)?;
    let amm_coin_mint = Pubkey::from_str(&pool_data.token_mint)?;
    let amm_pc_mint = SOL_MINT;
    // maintnet: 7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5
    // devnet: 3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR
    let create_fee_destination = Pubkey::from_str("7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5")?;

    let wallet = Keypair::from_base58_string(&pool_data.deployer_key);

    let client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let token_accounts = client
        .get_token_accounts_by_owner(
            &wallet.pubkey(),
            TokenAccountsFilter::Mint(Pubkey::from_str(&pool_data.token_mint)?),
        )
        .await?;

    let mut base_pc_amount = 0;
    for token_account in token_accounts {
        base_pc_amount = token_account.account.lamports;
    }

    let base_pc_amount_percentage = token_percentage();

    let input_pc_amount =
        sol_to_lamports(lamports_to_sol(base_pc_amount) * base_pc_amount_percentage);

    let input_coin_amount = liq_amount();
    // generate amm keys
    let amm_keys = get_amm_pda_keys(
        &amm_program,
        &market_program,
        &market,
        &amm_coin_mint,
        &amm_pc_mint,
    );
    // build initialize instruction
    let build_init_instruction = initialize_amm_pool(
        &amm_program,
        &amm_keys,
        &create_fee_destination,
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
        0,
        input_pc_amount,
        input_coin_amount,
    )?;

    // let pools_keys = PoolKeysSniper{
    //     id: amm_keys.amm_pool,
    //     base_mint: amm_coin_mint,
    //     quote_mint: amm_pc_mint,
    //     lp_mint: amm_keys.amm_lp_mint,
    //     authority: amm_keys.amm_authority,
    //     base_decimals: 6,
    //     quote_decimals: 9,
    //     lp_decimals: 6,
    //     version: 2,
    //     program_id: amm_program,
    //     open_orders: amm_keys.amm_open_order,
    //     target_orders: amm_keys.amm_target,
    //     base_vault: amm_keys.amm_coin_vault,
    //     quote_vault: amm_keys.amm_pc_vault,
    //     withdraw_queue: Pubkey::default(),
    //     lp_vault:

    // }

    Ok((build_init_instruction, amm_keys.amm_pool, amm_keys))
}

pub fn token_percentage() -> f64 {
    let theme = theme();
    let t = Input::new("Enter the percentage of token to be added")
        .placeholder("90%...")
        .theme(&theme)
        .prompt("Input: ");

    let amount = t.run().expect("error running input");

    amount.parse::<f64>().unwrap() / 100.0
}
pub fn liq_amount() -> u64 {
    let theme = theme();
    let t = Input::new("Enter the Liquidity Amount")
        .placeholder("5...")
        .theme(&theme)
        .prompt("Input: ");

    let tokens = t.run().expect("error running input");

    tokens.parse::<u64>().unwrap()
}
