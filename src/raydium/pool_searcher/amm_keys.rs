use std::{str::FromStr, sync::Arc};

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

use crate::{
    raydium::{
        subscribe::PoolKeysSniper,
        utils::utils::{
            market_authority, program_address, LIQUIDITY_STATE_LAYOUT_V4, MARKET_STATE_LAYOUT_V3,
            SPL_MINT_LAYOUT,
        },
    },
    rpc::{rpc_key, HTTP_CLIENT},
};

pub async fn pool_keys_fetcher(id: Pubkey) -> eyre::Result<PoolKeysSniper> {
    let http_client = HTTP_CLIENT.lock().unwrap();
    let rpc_client = http_client.get("http_client").unwrap();
    let mut retries = 0;
    let max_retries = 1000;
    let mut account = None;

    while account.is_none() && retries < max_retries {
        match rpc_client.get_account(&id).await {
            Ok(acc) => account = Some(acc),
            Err(_) => {
                retries += 1;
                continue;
            }
        }
    }

    let account = match account {
        Some(acc) => acc,
        None => return Err(eyre::eyre!("Account not found after maximum retries")),
    };

    let data = account.clone().data;
    let info = LIQUIDITY_STATE_LAYOUT_V4::decode(&mut &data[..])?;
    let marketid = info.marketId;

    let market_account = rpc_client.get_account(&marketid).await?;
    let market_data = market_account.data;

    let market_info = MARKET_STATE_LAYOUT_V3::decode(&mut &market_data[..])?;
    let lp_mint = info.lpMint;

    let lp_mint_account = match rpc_client.get_account(&lp_mint).await {
        Ok(acc) => acc,
        Err(_) => return Err(eyre::eyre!("Account not found after maximum retries")),
    };
    let lp_mint_data = lp_mint_account.data;

    let lp_mint_info = SPL_MINT_LAYOUT::decode(&mut &lp_mint_data[..])?;

    let pool_keys = PoolKeysSniper {
        id: id,
        base_mint: info.baseMint,
        quote_mint: info.quoteMint,
        lp_mint: info.lpMint,
        base_decimals: info.baseDecimal as u8,
        quote_decimals: info.quoteDecimal as u8,
        lp_decimals: lp_mint_info.decimals,
        version: 4,
        program_id: account.owner,
        authority: program_address(&account.owner).await?,
        open_orders: info.openOrders,
        target_orders: info.targetOrders,
        base_vault: info.baseVault,
        quote_vault: info.quoteVault,
        withdraw_queue: info.withdrawQueue,
        lp_vault: info.lpVault,
        market_version: 3,
        market_program_id: info.marketProgramId,
        market_id: info.marketId,
        market_authority: market_authority(rpc_client, market_info.quoteVault).await,
        market_base_vault: market_info.baseVault,
        market_quote_vault: market_info.quoteVault,
        market_bids: market_info.bids,
        market_asks: market_info.asks,
        market_event_queue: market_info.eventQueue,
        lookup_table_account: Pubkey::default(),
    };

    Ok(pool_keys)
}
