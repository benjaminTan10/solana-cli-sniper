use std::str::FromStr;

use crate::{
    app::theme,
    env::minter::PoolDataSettings,
    instruction::instruction::{get_amm_pda_keys, initialize_amm_pool, AmmKeys, SOL_MINT},
    raydium::swap::instructions::SOLC_MINT,
    rpc::HTTP_CLIENT,
};
use demand::Input;
use log::info;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{
    instruction::Instruction,
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey,
    system_instruction::{self, create_account_with_seed},
};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::instruction::{initialize_account, sync_native};

pub const AMM_PROGRAM: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

pub async fn pool_ixs(
    pool_data: PoolDataSettings,
) -> eyre::Result<(Vec<Instruction>, Pubkey, AmmKeys)> {
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
        .get_token_accounts_by_owner(&wallet.pubkey(), TokenAccountsFilter::Mint(SOLC_MINT))
        .await?;

    let mut base_pc_amount = 0;
    for token_account in token_accounts {
        base_pc_amount = token_account.account.lamports;

        info!("Tokens Balance: {}", base_pc_amount);
    }

    let sol_amount = liq_amount();
    let percentage = token_percentage();

    let input_pc_amount = sol_to_lamports(lamports_to_sol(base_pc_amount) * percentage);

    info!("SOL Amount: {}", sol_amount);
    info!("Tokens Amount: {}", input_pc_amount);
    // generate amm keys
    let amm_keys = get_amm_pda_keys(
        &AMM_PROGRAM,
        &market_program,
        &market,
        &amm_coin_mint,
        &amm_pc_mint,
    );

    let mut pool_inx = vec![];

    // let account = create_associated_token_account(
    //     &wallet.pubkey(),
    //     &wallet.pubkey(),
    //     &amm_keys.amm_pc_mint,
    //     &spl_token::id(),
    // );

    // let inx = create_account_with_seed(
    //     &wallet.pubkey(),
    //     &get_associated_token_address(&wallet.pubkey(), &amm_keys.amm_pc_mint),
    //     &wallet.pubkey(),
    //     "9kQCXsHAYio4GFURzBWpEQc97pVXVUdD",
    //     1002039280,
    //     165,
    //     &wallet.pubkey(),
    // );

    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let space = 0;
    let rent_exemption_amount = connection
        .get_minimum_balance_for_rent_exemption(space)
        .await
        .unwrap();

    let new_account_keypair = Keypair::new();
    let new_account_pubkey = Signer::pubkey(&new_account_keypair);

    let create_account_ix = system_instruction::create_account(
        &wallet.pubkey(),
        &new_account_pubkey,
        rent_exemption_amount,
        space as u64,
        &wallet.pubkey(),
    );

    let init = initialize_account(
        &spl_token::id(),
        &new_account_pubkey,
        &SOLC_MINT,
        &wallet.pubkey(),
    )?;

    // let user_token_source = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

    // let wrap_sol = system_instruction::transfer(&wallet.pubkey(), &user_token_source, sol_amount);

    // let sync_native = match sync_native(&spl_token::id(), &user_token_source) {
    //     Ok(sync_native) => sync_native,
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         panic!("Error: {}", e);
    //     }
    // };

    println!(
        "Coin: {}\nPC: {}\nLP: {}",
        amm_keys.amm_coin_mint, amm_keys.amm_pc_mint, amm_keys.amm_lp_mint
    );

    let token = spl_associated_token_account::get_associated_token_address(
        &wallet.pubkey(),
        &amm_keys.amm_pc_mint,
    );

    println!("Token: {}", token);
    // build initialize instruction
    let build_init_instruction = initialize_amm_pool(
        &AMM_PROGRAM,
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
        sol_amount,
        input_pc_amount,
    )?;

    pool_inx.push(create_account_ix);
    pool_inx.push(init);
    // pool_inx.push(wrap_sol);
    // pool_inx.push(sync_native);
    pool_inx.push(build_init_instruction);

    Ok((pool_inx, amm_keys.amm_pool, amm_keys))
}

pub fn token_percentage() -> f64 {
    let theme = theme();
    let t = Input::new("Enter the percentage of tokens:")
        .placeholder("eg. 90%...")
        .theme(&theme)
        .prompt("Input: ");

    let amount = t.run().expect("error running input");

    amount.parse::<f64>().unwrap() / 100.0
}
pub fn liq_amount() -> u64 {
    let theme = theme();
    let t = Input::new("Enter the Liquidity Amount in SOL")
        .placeholder("eg. 5 sol...")
        .theme(&theme)
        .prompt("Input: ");

    let tokens = t.run().expect("error running input");

    tokens.parse::<u64>().unwrap()
}
