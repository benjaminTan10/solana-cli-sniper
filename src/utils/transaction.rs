use {
    crate::{
        env::SettingsConfig, liquidity::utils::tip_account,
        raydium_amm::swap::swapper::auth_keypair,
    },
    jito_protos::searcher::SubscribeBundleResultsRequest,
    jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation},
    log::{error, info},
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig},
    solana_sdk::{
        native_token::sol_to_lamports,
        signature::Keypair,
        signer::Signer,
        system_instruction::transfer,
        transaction::{Transaction, VersionedTransaction},
    },
    std::sync::Arc,
};

pub async fn send_transaction(
    settings_config: SettingsConfig,
    transaction: VersionedTransaction,
) -> eyre::Result<()> {
    let wallet = Keypair::from_base58_string(&settings_config.engine.payer_keypair);
    let rpc_client = Arc::new(RpcClient::new(settings_config.network.rpc_url));

    if settings_config.engine.use_bundles {
        info!("Building Bundle");

        let tip_txn = VersionedTransaction::from(Transaction::new_signed_with_payer(
            &[transfer(
                &wallet.pubkey(),
                &tip_account(),
                sol_to_lamports(settings_config.trading.bundle_tip),
            )],
            Some(&wallet.pubkey()),
            &[&wallet],
            rpc_client.get_latest_blockhash().await.unwrap(),
        ));

        let bundle_txn = vec![transaction, tip_txn];
        let mut searcher_client = get_searcher_client(
            &settings_config.network.block_engine_url,
            &Arc::new(auth_keypair()),
        )
        .await
        .unwrap();
        let mut bundle_results_subscription = searcher_client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        match send_bundle_with_confirmation(
            &bundle_txn,
            &rpc_client,
            &mut searcher_client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: {}", e);
            }
        }

        std::mem::drop(bundle_results_subscription);
    } else {
        info!("Sending Transaction");
        let config = RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        };

        if settings_config.trading.spam {
            let mut counter = 0;
            while counter < settings_config.trading.spam_count {
                let result = match rpc_client
                    .send_transaction_with_config(&transaction, config)
                    .await
                {
                    Ok(x) => x,
                    Err(e) => {
                        error!("Error: {:?}", e);
                        return Ok(());
                    }
                };

                info!("Transaction Sent {:?}", result);
                counter += 1;
            }
        } else {
            let result = match rpc_client
                .send_transaction_with_config(&transaction, config)
                .await
            {
                Ok(x) => x,
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            };

            info!("Transaction Sent {:?}", result);
        }
    }

    Ok(())
}
