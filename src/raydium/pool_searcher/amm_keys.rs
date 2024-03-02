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
    rpc::rpc_key,
};

pub async fn pool_keys_fetcher(
    id: String,
) -> eyre::Result<(PoolKeysSniper, LIQUIDITY_STATE_LAYOUT_V4)> {
    let rpc_client = RpcClient::new(rpc_key());
    let mut retries = 0;
    let max_retries = 1000;
    let mut account = None;

    while account.is_none() && retries < max_retries {
        match rpc_client.get_account(&Pubkey::from_str(&id)?).await {
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
    let marketid = info.marketId.to_string();

    let market_account = rpc_client
        .get_account(&Pubkey::from_str(&marketid)?)
        .await?;
    let market_data = market_account.data;

    let market_info = MARKET_STATE_LAYOUT_V3::decode(&mut &market_data[..])?;
    let lp_mint = info.lpMint.to_string();

    let lp_mint_account = match rpc_client.get_account(&Pubkey::from_str(&lp_mint)?).await {
        Ok(acc) => acc,
        Err(_) => return Err(eyre::eyre!("Account not found after maximum retries")),
    };
    let lp_mint_data = lp_mint_account.data;

    let lp_mint_info = SPL_MINT_LAYOUT::decode(&mut &lp_mint_data[..])?;

    let pool_keys = PoolKeysSniper {
        id: id,
        base_mint: info.baseMint.to_string(),
        quote_mint: info.quoteMint.to_string(),
        lp_mint: info.lpMint.to_string(),
        base_decimals: info.baseDecimal as u8,
        quote_decimals: info.quoteDecimal as u8,
        lp_decimals: lp_mint_info.decimals,
        version: 4,
        program_id: account.owner.to_string(),
        authority: program_address(&account.owner).await?.to_string(),
        open_orders: info.openOrders.to_string(),
        target_orders: info.targetOrders.to_string(),
        base_vault: info.baseVault.to_string(),
        quote_vault: info.quoteVault.to_string(),
        withdraw_queue: info.withdrawQueue.to_string(),
        lp_vault: info.lpVault.to_string(),
        market_version: 3,
        market_program_id: info.marketProgramId.to_string(),
        market_id: info.marketId.to_string(),
        market_authority: market_authority(
            Arc::new(rpc_client),
            &market_info.quoteVault.to_string(),
        )
        .await,
        market_base_vault: market_info.baseVault.to_string(),
        market_quote_vault: market_info.quoteVault.to_string(),
        market_bids: market_info.bids.to_string(),
        market_asks: market_info.asks.to_string(),
        market_event_queue: market_info.eventQueue.to_string(),
        lookup_table_account: Some(Pubkey::default().to_string()),
    };

    Ok((pool_keys, info))
}
