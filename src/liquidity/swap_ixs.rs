use std::str::FromStr;

use solana_sdk::{
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};

use crate::{
    env::minter::PoolDataSettings,
    instruction::instruction::{
        get_keys_for_market, swap, AmmKeys, MarketPubkeys, PoolKeysSniper, SOL_MINT,
    },
    rpc::HTTP_CLIENT,
};

use super::{pool_ixs::AMM_PROGRAM, utils::JitoPoolData};

pub fn swap_ixs(
    server_data: PoolDataSettings,
    amm_keys: AmmKeys,
    market_keys: MarketPubkeys,
    wallet: &Keypair,
    amount_in: u64,
    out: bool,
    user_token_source: Pubkey,
) -> eyre::Result<Instruction> {
    let buyer_keypair = Keypair::from_base58_string(&server_data.buyer_key);
    let buyer_wallet = wallet;
    // if out {
    //     buyer_wallet = &buyer_keypair;
    // }

    let user_token_destination = get_associated_token_address(
        &buyer_wallet.pubkey(),
        &Pubkey::from_str(&server_data.token_mint)?,
    );

    // build swap instruction
    let build_swap_instruction = swap(
        &AMM_PROGRAM,
        &amm_keys,
        &market_keys,
        &buyer_wallet.pubkey(),
        &user_token_source,
        &user_token_destination,
        amount_in,
        1,
        out,
    )?;

    Ok(build_swap_instruction)
}

pub async fn atas_creation(
    server_data: JitoPoolData,
    pool_keys: PoolKeysSniper,
    wallets: Vec<Keypair>,
) -> eyre::Result<()> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let payer = Keypair::from_base58_string(&server_data.buyerPrivateKey);

    let base_atas = wallets
        .iter()
        .map(|wallet| get_associated_token_address(&wallet.pubkey(), &pool_keys.base_mint))
        .collect::<Vec<_>>();

    let quote_atas = wallets
        .iter()
        .map(|wallet| get_associated_token_address(&wallet.pubkey(), &pool_keys.quote_mint))
        .collect::<Vec<_>>();

    let create_base_atas = base_atas
        .iter()
        .map(|ata| {
            create_associated_token_account_idempotent(
                &payer.pubkey(),
                &ata,
                &pool_keys.base_mint,
                &spl_token::id(),
            )
        })
        .collect::<Vec<_>>();

    let create_quote_atas = quote_atas
        .iter()
        .map(|ata| {
            create_associated_token_account_idempotent(
                &payer.pubkey(),
                &ata,
                &pool_keys.quote_mint,
                &spl_token::id(),
            )
        })
        .collect::<Vec<_>>();

    let mut instructions = vec![];
    instructions.extend(create_base_atas);
    instructions.extend(create_quote_atas);

    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();
    let mut current_size = 0;

    for instruction in instructions {
        let instruction_size = std::mem::size_of_val(&instruction);
        if current_size + instruction_size > 1200 {
            chunks.push(current_chunk);
            current_chunk = Vec::new();
            current_size = 0;
        }
        current_chunk.push(instruction);
        current_size += instruction_size;
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let mut bundle = vec![];

    for chunk in chunks {
        let versioned_msg = VersionedMessage::V0(Message::try_compile(
            &payer.pubkey(),
            &chunk,
            &[],
            recent_blockhash,
        )?);
        let transaction = VersionedTransaction::try_new(versioned_msg, &[&payer])?;

        bundle.push(transaction);
    }

    Ok(())
}

pub async fn load_pool_keys(amm_pool: Pubkey, amm_keys: AmmKeys) -> eyre::Result<MarketPubkeys> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    // let amm_keys = load_amm_keys(&connection, &AMM_PROGRAM, &amm_pool)
    //     .await
    //     .unwrap();
    // load market keys
    let market_keys = get_keys_for_market(&connection, &amm_keys.market_program, &amm_keys.market)
        .await
        .unwrap();

    Ok(market_keys)
}
