use std::{error::Error, str::FromStr, sync::Arc};

use demand::Input;
use log::{error, info};
use rand::{Rng, SeedableRng};
use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount, address_lookup_table_account,
    commitment_config::CommitmentLevel, native_token::sol_to_lamports, pubkey::Pubkey,
    signature::Keypair, signer::Signer, transaction::VersionedTransaction,
};

use crate::{
    app::theme,
    env::{load_config, minter::load_minter_settings},
    raydium_amm::{
        bundles::swap_instructions::volume_swap_base_in,
        pool_searcher::amm_keys::pool_keys_fetcher,
        subscribe::PoolKeysSniper,
        swap::instructions::{swap_base_out, token_price_data, SOLC_MINT},
    },
    user_inputs::{amounts::priority_fee, tokens::token_env},
};

pub struct VolumeBotSettings {
    pub buy_amount: u64,
    pub priority_fee: u64,
    pub wallet: Keypair,
}

pub async fn buy_amount(input: &str) -> Result<u64, Box<dyn Error>> {
    let theme = theme();
    let t = Input::new(format!("{}:", input))
        .placeholder("0.01")
        .theme(&theme)
        .prompt("Input: ");

    let string = t.run().expect("error running input");

    let amount = sol_to_lamports(string.parse::<f64>()?);

    Ok(amount)
}

pub async fn generate_volume() -> Result<(), Box<dyn Error>> {
    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error 1: {:?}", e);
            return Ok(());
        }
    };

    let rpc_client = RpcClient::new(args.network.rpc_url.to_string());
    let data = load_minter_settings().await?;

    let lut_key = match Pubkey::from_str(&data.volume_lut_key) {
        Ok(x) => x,
        Err(e) => {
            error!("Error 2: {:?}", e);
            return Ok(());
        }
    };

    let mut raw_account = None;

    while raw_account.is_none() {
        match rpc_client.get_account(&lut_key).await {
            Ok(account) => raw_account = Some(account),
            Err(e) => {
                eprintln!("Error getting LUT account: {}, retrying...", e);
            }
        }
    }

    let raw_account = raw_account.unwrap();
    let address_lookup_table = AddressLookupTable::deserialize(&raw_account.data)?;
    let address_lookup_table_account = AddressLookupTableAccount {
        key: lut_key,
        addresses: address_lookup_table.addresses.to_vec(),
    };

    let min_amount = buy_amount("Min Amount").await?;
    let max_amount = buy_amount("Max Amount").await?;

    let priority_fee = priority_fee().await;
    let pool_address = token_env("Pool Address").await;

    let data = load_minter_settings().await?;

    let pool_keys = pool_keys_fetcher(pool_address).await?;

    // info!(
    //     "Pool Keys: {}",
    //     serde_json::to_string_pretty(&pool_keys).unwrap()
    // );
    let wallet = Keypair::from_base58_string(&data.buyer_key);
    let rpc_client = RpcClient::new(args.network.rpc_url.to_string());

    let mut rng = rand::rngs::StdRng::from_entropy();
    let buy_amount: u64 = rng.gen_range(min_amount..=max_amount);
    let volume_settings = VolumeBotSettings {
        buy_amount,
        priority_fee,
        wallet,
    };

    let _ = match volume_round(
        Arc::new(rpc_client),
        pool_keys.clone(),
        volume_settings,
        address_lookup_table_account.clone(),
    )
    .await
    {
        Ok(x) => x,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(());
        }
    };
    Ok(())
}

pub async fn volume_round(
    rpc_client: Arc<RpcClient>,
    pool_keys: PoolKeysSniper,
    volume_bot: VolumeBotSettings,
    address_lookup_table_account: AddressLookupTableAccount,
) -> Result<(), Box<dyn Error>> {
    info!("Volume Round ..");
    let wallet = Arc::new(volume_bot.wallet);
    let user_source_owner = wallet.pubkey();

    let token_address = if pool_keys.base_mint == SOLC_MINT {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };
    let tokens_amount = match token_price_data(
        rpc_client.clone(),
        pool_keys.clone(),
        wallet.clone(),
        volume_bot.buy_amount,
        crate::raydium_amm::swap::instructions::SwapDirection::Coin2PC,
    )
    .await
    {
        Ok(x) => x,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(());
        }
    };

    let tokens_amount = tokens_amount * 999 / 1000;

    info!("Swap amount out: {}", tokens_amount);

    let transaction_main_instructions = volume_swap_base_in(
        &Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap(),
        &pool_keys.id,
        &pool_keys.authority,
        &pool_keys.open_orders,
        &pool_keys.target_orders,
        &pool_keys.base_vault,
        &pool_keys.quote_vault,
        &pool_keys.market_program_id,
        &pool_keys.market_id,
        &pool_keys.market_bids,
        &pool_keys.market_asks,
        &pool_keys.market_event_queue,
        &pool_keys.market_base_vault,
        &pool_keys.market_quote_vault,
        &pool_keys.market_authority,
        &user_source_owner,
        &user_source_owner,
        &token_address,
        volume_bot.buy_amount,
        tokens_amount as u64,
        volume_bot.priority_fee,
        rpc_client.clone(),
    )
    .await?;

    // transaction_main_instructions.extend(swap_out_instructions);

    let config = CommitmentLevel::Confirmed;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &transaction_main_instructions,
        &[],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let frontrun_tx = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let config = RpcSendTransactionConfig {
        skip_preflight: true,
        ..Default::default()
    };
    let txn = rpc_client
        .send_and_confirm_transaction_with_spinner(&frontrun_tx)
        .await;

    match txn {
        Ok(x) => {
            info!("Swap in Transaction: {:?}", x);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }

    let swap_out_instructions = swap_base_out(
        &Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap(),
        &pool_keys.id,
        &pool_keys.authority,
        &pool_keys.open_orders,
        &pool_keys.target_orders,
        &pool_keys.base_vault,
        &pool_keys.quote_vault,
        &pool_keys.market_program_id,
        &pool_keys.market_id,
        &pool_keys.market_bids,
        &pool_keys.market_asks,
        &pool_keys.market_event_queue,
        &pool_keys.market_base_vault,
        &pool_keys.market_quote_vault,
        &pool_keys.market_authority,
        &user_source_owner,
        &user_source_owner,
        &token_address,
        tokens_amount as u64,
        0,
        volume_bot.priority_fee,
    )
    .await?;

    info!("Sending Transaction...!");

    // transaction_main_instructions.extend(swap_out_instructions);

    let config = CommitmentLevel::Confirmed;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &swap_out_instructions,
        &[address_lookup_table_account],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };
    let frontrun_tx = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let config = RpcSendTransactionConfig {
        skip_preflight: true,
        ..Default::default()
    };
    let txn = rpc_client
        .send_and_confirm_transaction_with_spinner(&frontrun_tx)
        .await;

    match txn {
        Ok(x) => {
            info!("Swap out Transaction: {:?}", x);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
    Ok(())
}
