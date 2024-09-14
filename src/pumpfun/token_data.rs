use {
    crate::{
        app::config_init::get_config,
        pumpfun::instructions::{
            instructions::{
                calculate_buy_price, calculate_sell_price, get_bonding_curve, PUMP_PROGRAM,
            },
            pumpfun_program::accounts::BondingCurve,
        },
        raydium_amm::{
            subscribe::PoolKeysSniper,
            swap::{
                instructions::{token_price_data, SwapDirection},
                raydium_amm_sniper::clear_previous_line,
                raydium_swap_in::sell_tokens,
            },
        },
        rpc::HTTP_CLIENT,
    },
    borsh::BorshDeserialize,
    log::{error, info},
    solana_client::{
        nonblocking::rpc_client::{self, RpcClient},
        rpc_request::TokenAccountsFilter,
    },
    solana_program::pubkey::Pubkey,
    solana_sdk::{native_token::lamports_to_sol, signature::Keypair, signer::Signer},
    spl_associated_token_account::get_associated_token_address,
    std::{str::FromStr, sync::Arc, time::Duration},
    tokio::{sync::mpsc, time::sleep},
};

pub async fn price_logger(
    stop_rx: &mut mpsc::Receiver<()>,
    amount_in: u64,
    pool_keys: PoolKeysSniper,
    wallet: Arc<Keypair>,
) {
    let config = get_config().await.unwrap();
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    loop {
        if let Ok(_) = stop_rx.try_recv() {
            sleep(Duration::from_secs(10)).await;
            break;
        }

        let mut token_balance = 0;
        let rpc_client_clone = rpc_client.clone();
        let pool_keys_clone = pool_keys.clone();
        let wallet_clone = Arc::clone(&wallet);
        let token_accounts = match rpc_client_clone
            .get_token_accounts_by_owner(
                &wallet_clone.pubkey(),
                TokenAccountsFilter::Mint(pool_keys_clone.base_mint),
            )
            .await
        {
            Ok(token_accounts) => token_accounts,
            Err(e) => {
                error!("Error: {:?}", e);
                break;
            }
        };

        for rpc_keyed_account in &token_accounts {
            let pubkey = &rpc_keyed_account.pubkey;
            //convert to pubkey
            let pubkey = match Pubkey::from_str(&pubkey) {
                Ok(pubkey) => pubkey,
                Err(e) => {
                    eprintln!("Failed to parse pubkey: {}", e);
                    break;
                }
            };

            let balance = match rpc_client_clone.get_token_account_balance(&pubkey).await {
                Ok(balance) => balance,
                Err(e) => {
                    eprintln!("Failed to get token account balance: {}", e);
                    break;
                }
            };
            let lamports = match balance.amount.parse::<u64>() {
                Ok(lamports) => lamports,
                Err(e) => {
                    eprintln!("Failed to parse balance: {}", e);
                    break;
                }
            };

            token_balance = lamports;

            if lamports != 0 {
                break;
            }

            std::thread::sleep(Duration::from_secs(1));
        }

        let price = match token_price_data(
            rpc_client_clone,
            pool_keys_clone,
            wallet_clone,
            token_balance,
            SwapDirection::PC2Coin,
        )
        .await
        {
            Ok(price) => price,
            Err(e) => {
                error!("Error: {:?}", e);
                break;
            }
        };

        let total_value = lamports_to_sol(price as u64);
        let profit_percentage =
            ((total_value - lamports_to_sol(amount_in)) / lamports_to_sol(amount_in)) * 100.0;

        clear_previous_line();
        info!(
            "Aped: {:.3} Sol | Worth {:.4} Sol | Profit {:.2}%",
            lamports_to_sol(amount_in),
            total_value,
            profit_percentage
        );

        if profit_percentage >= config.trading.profit_threshold_percentage {
            let _ = match sell_tokens(100, pool_keys.clone()).await {
                Ok(_) => {}
                Err(e) => {
                    info!("Error: {}", e);
                }
            };
        } else if profit_percentage <= config.trading.loss_threshold_percentage {
            let _ = match sell_tokens(100, pool_keys.clone()).await {
                Ok(_) => {}
                Err(e) => {
                    info!("Error: {}", e);
                }
            };
        }

        // Sleep for a while
        sleep(Duration::from_millis(500)).await;
    }
}

pub async fn bonding_curve_fetcher(token: Pubkey, token_amount: u64) -> eyre::Result<u128> {
    let config = get_config().await?;
    let rpc_client = RpcClient::new(config.network.rpc_url);

    let bonding_curve_pda = get_bonding_curve(token, &PUMP_PROGRAM);

    let account_data = rpc_client.get_account_data(&bonding_curve_pda).await?;
    let sliced_data: &mut &[u8] = &mut account_data.as_slice();
    let reserves = BondingCurve::deserialize_reader(sliced_data)?;

    let reserves = (
        reserves.real_token_reserves as u128,
        reserves.virtual_sol_reserves as u128,
        reserves.real_sol_reserves as u128,
    );

    let result = calculate_sell_price(token_amount as u128, reserves);

    Ok(result.0)
}
