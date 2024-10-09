use solana_sdk::transaction::VersionedTransaction;

use crate::{
    app::config_init::get_config,
    jupiter::jup_utils::{quote, swap, QuoteConfig, Swap, SwapRequest},
};

use super::interface::RouteKeys;

use {
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        hash::Hash,
        pubkey,
        signature::{Keypair, Signer},
    },
    spl_token::{amount_to_ui_amount, ui_amount_to_amount},
};

pub async fn jup_swap(route: RouteKeys) -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().await?;

    let sol = pubkey!("So11111111111111111111111111111111111111112");
    let output_mint = pubkey!(route.destination_mint);

    let keypair = Keypair::from_base58_string(&config.engine.payer_keypair);

    let rpc_client = RpcClient::new_with_commitment(
        config.network.rpc_url.into(),
        CommitmentConfig::confirmed(),
    );

    let msol_token_address =
        spl_associated_token_account::get_associated_token_address(&keypair.pubkey(), &output_mint);
    println!(
        "Pre-swap SOL balance: {}",
        amount_to_ui_amount(rpc_client.get_balance(&keypair.pubkey()).await?, 9)
    );
    println!(
        "Pre-swap mSOL balance: {}",
        amount_to_ui_amount(
            rpc_client
                .get_token_account_balance(&msol_token_address)
                .await?
                .amount
                .parse::<u64>()?,
            9
        )
    );

    let slippage_bps = 100;
    let only_direct_routes = true;
    let quotes = quote(
        sol,
        output_mint,
        ui_amount_to_amount(config.trading.buy_amount, 9),
        QuoteConfig {
            only_direct_routes,
            slippage_bps: Some(slippage_bps),
            ..QuoteConfig::default()
        },
    )
    .await?;

    let route = quotes.route_plan[0]
        .swap_info
        .label
        .clone()
        .unwrap_or_else(|| "Unknown DEX".to_string());
    println!(
        "Quote: {} SOL for {} mSOL via {} (worst case with slippage: {}). Impact: {:.2}%",
        amount_to_ui_amount(quotes.in_amount, 9),
        amount_to_ui_amount(quotes.out_amount, 9),
        route,
        amount_to_ui_amount(quotes.other_amount_threshold, 9),
        quotes.price_impact_pct * 100.
    );

    let request: SwapRequest = SwapRequest::new(keypair.pubkey(), quotes.clone());

    let Swap {
        mut swap_transaction,
        last_valid_block_height: _,
    } = swap(request).await?;

    let recent_blockhash_for_swap: Hash = rpc_client.get_latest_blockhash().await?;
    swap_transaction
        .message
        .set_recent_blockhash(recent_blockhash_for_swap); // Updating to latest blockhash to not error out

    let swap_transaction = VersionedTransaction::try_new(swap_transaction.message, &[&keypair])?;
    println!(
        "Simulating swap transaction: {}",
        swap_transaction.signatures[0]
    );
    let response = rpc_client.simulate_transaction(&swap_transaction).await?;
    println!("  {:#?}", response.value);
    println!("Sending transaction: {}", swap_transaction.signatures[0]);
    let _ = rpc_client
        .send_and_confirm_transaction_with_spinner(&swap_transaction)
        .await?;

    println!(
        "Post-swap SOL balance: {}",
        amount_to_ui_amount(rpc_client.get_balance(&keypair.pubkey()).await?, 9)
    );
    println!(
        "Post-swap mSOL balance: {}",
        amount_to_ui_amount(
            rpc_client
                .get_token_account_balance(&msol_token_address)
                .await?
                .amount
                .parse::<u64>()?,
            9
        )
    );

    Ok(())
}
