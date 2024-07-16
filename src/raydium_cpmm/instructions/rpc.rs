use anyhow::{anyhow, Result};
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
    rpc_request::RpcRequest,
    rpc_response::{RpcResult, RpcSimulateTransactionResult},
};
use solana_sdk::{
    account::Account,
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    program_pack::Pack as TokenPack,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};
use std::{convert::Into, sync::Arc};

use crate::{
    env::EngineSettings, liquidity::utils::tip_account, raydium_amm::swap::swapper::auth_keypair,
};

pub async fn simulate_transaction(
    client: &solana_client::nonblocking::rpc_client::RpcClient,
    transaction: &VersionedTransaction,
    sig_verify: bool,
    cfg: CommitmentConfig,
) -> RpcResult<RpcSimulateTransactionResult> {
    let serialized_encoded = bs58::encode(bincode::serialize(transaction).unwrap()).into_string();
    client
        .send(
            RpcRequest::SimulateTransaction,
            serde_json::json!([serialized_encoded, {
                "sigVerify": sig_verify, "commitment": cfg.commitment
            }]),
        )
        .await
}

// 1 : Processed, 2: Confirmed, 3: Finalized
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Commitment {
    Processed,
    Confirmed,
    Finalized,
}

pub async fn send_txn(
    client: &solana_client::nonblocking::rpc_client::RpcClient,
    txn: &Transaction,
    wait_confirm: bool,
) -> Result<Signature> {
    //simulate transaction
    // let sim_result = simulate_transaction(client, txn, true, CommitmentConfig::processed()).await?;

    // println!("Simulate result: {:?}", sim_result);

    Ok(client
        .send_and_confirm_transaction_with_spinner_and_config(
            txn,
            if wait_confirm {
                CommitmentConfig::confirmed()
            } else {
                CommitmentConfig::processed()
            },
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..RpcSendTransactionConfig::default()
            },
        )
        .await?)
}

pub fn get_token_account<T: TokenPack>(client: &RpcClient, addr: &Pubkey) -> Result<T> {
    let account = client
        .get_account_with_commitment(addr, CommitmentConfig::processed())?
        .value
        .map_or(Err(anyhow!("Account not found")), Ok)?;
    T::unpack_from_slice(&account.data).map_err(Into::into)
}

pub fn get_multiple_accounts(
    client: &RpcClient,
    pubkeys: &[Pubkey],
) -> Result<Vec<Option<Account>>> {
    Ok(client.get_multiple_accounts(pubkeys)?)
}

pub async fn transaction_handler(
    rpc_client: &Arc<solana_client::nonblocking::rpc_client::RpcClient>,
    payer: Arc<Keypair>,
    swap_instructions: Vec<Instruction>,
    bundle_tip: u64,
    args: &EngineSettings,
) -> eyre::Result<()> {
    let latest_blockhash = rpc_client.get_latest_blockhash().await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &payer.pubkey(),
        &swap_instructions,
        &[],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };
    let transaction = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&payer],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let sim_result = simulate_transaction(
        rpc_client,
        &transaction,
        true,
        CommitmentConfig::processed(),
    )
    .await?;

    println!("Simulate result: {:?}", sim_result);

    if args.use_bundles {
        info!("Building Bundle");

        let tip_txn = VersionedTransaction::from(Transaction::new_signed_with_payer(
            &[transfer(&payer.pubkey(), &tip_account(), bundle_tip)],
            Some(&payer.pubkey()),
            &[&payer],
            rpc_client.get_latest_blockhash().await.unwrap(),
        ));

        let bundle_txn = vec![transaction, tip_txn];

        let mut searcher_client =
            get_searcher_client(&args.block_engine_url, &Arc::new(auth_keypair())).await?;

        let mut bundle_results_subscription = searcher_client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        match send_bundle_with_confirmation(
            &bundle_txn,
            rpc_client,
            &mut searcher_client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err(eyre::eyre!("Error: {}", e));
            }
        }

        std::mem::drop(bundle_results_subscription);
    } else {
        info!("Sending Transaction");
        let config = RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        };

        if args.spam {
            let mut counter = 0;
            while counter < args.spam_count {
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
