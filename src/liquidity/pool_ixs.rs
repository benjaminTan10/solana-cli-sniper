use std::str::FromStr;

use crate::{
    app::theme,
    env::minter::PoolDataSettings,
    instruction::instruction::{get_amm_pda_keys, initialize_amm_pool, AmmKeys, SOL_MINT},
    raydium::swap::instructions::SOLC_MINT,
    rpc::HTTP_CLIENT,
};
use demand::{Confirm, Input};
use log::info;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{
    instruction::Instruction,
    native_token::{lamports_to_sol, sol_to_lamports},
    program_pack::Pack,
    pubkey::{Pubkey, MAX_SEED_LEN},
    system_instruction::create_account_with_seed,
};

use solana_sdk::{signature::Keypair, signer::Signer};
use spl_token::instruction::initialize_account;

pub const AMM_PROGRAM: Pubkey = solana_sdk::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

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

    let associated_token = spl_associated_token_account::get_associated_token_address(
        &wallet.pubkey(),
        &Pubkey::from_str(&pool_data.token_mint)?,
    );

    let token_accounts = client.get_token_account_balance(&associated_token).await?;

    let base_pc_amount = token_accounts.amount.parse::<u64>()?;

    println!("Base PC Amount: {}", base_pc_amount);

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

    let (pubkey, seed) = generate_pubkey(wallet.pubkey()).await?;

    // let account = create_associated_token_account(
    //     &wallet.pubkey(),
    //     &wallet.pubkey(),
    //     &amm_keys.amm_pc_mint,
    //     &spl_token::id(),
    // );

    println!("Seed: {}", seed);

    let inx = create_account_with_seed(
        &wallet.pubkey(),
        &pubkey,
        &wallet.pubkey(),
        &seed,
        sol_amount + 2039280,
        165,
        &spl_token::id(),
    );

    let init = initialize_account(&spl_token::id(), &pubkey, &SOLC_MINT, &wallet.pubkey())?;
    // let user_token_source = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

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
        // &spl_associated_token_account::get_associated_token_address(
        //     &wallet.pubkey(),
        //     &amm_keys.amm_pc_mint,
        // )
        &pubkey,
        &spl_associated_token_account::get_associated_token_address(
            &wallet.pubkey(),
            &amm_keys.amm_lp_mint,
        ),
        0,
        sol_amount,
        input_pc_amount,
    )?;

    pool_inx.push(inx);
    pool_inx.push(init);
    pool_inx.push(build_init_instruction);

    // let rent = connection
    //     .get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)
    //     .await?;

    // let create_account_instruction = create_account(
    //     &spl_token::id(),
    //     &pubkey,
    //     sol_amount,
    //     rent,
    //     &wallet.pubkey(),
    // );
    // let hash = connection.get_latest_blockhash().await?;

    // let message = Message::try_compile(&wallet.pubkey(), &pool_inx, &[], hash)?;
    // let txn = VersionedTransaction::try_new(VersionedMessage::V0(message), &[&wallet])?;

    // let message = Message::new(&[create_account_instruction], Some(&wallet.pubkey()));

    // let transaction = Transaction::new_signed_with_payer(
    //     &[message],
    //     Some(&wallet.pubkey()),
    //     &[&wallet],
    //     connection.get_recent_blockhash().await?.0,
    // );

    // connection.send_transaction(&transaction).await?;

    // let result = match connection
    //     .send_and_confirm_transaction_with_spinner(&txn)
    //     .await
    // {
    //     Ok(result) => result,
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         panic!("Error: {:?}", e);
    //     }
    // };

    // println!("Result: {:?}", result);

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

    let tokens = sol_to_lamports(tokens.parse::<f64>().unwrap());

    tokens
}

pub async fn generate_pubkey(from_public_key: Pubkey) -> eyre::Result<(Pubkey, String)> {
    let seed = Keypair::new()
        .pubkey()
        .to_string()
        .chars()
        .take(MAX_SEED_LEN)
        .collect::<String>();
    info!("Seed: {}", seed);
    let public_key = Pubkey::create_with_seed(&from_public_key, &seed, &spl_token::id())?;
    info!("Public Key: {}", public_key);
    Ok((public_key, seed))
}
