use std::{str::FromStr, sync::Arc};

use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
use solana_client::{
    nonblocking::rpc_client::{self, RpcClient},
    rpc_config::RpcSendTransactionConfig,
};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    native_token::sol_to_lamports,
    pubkey::{self, Pubkey},
    signature::Keypair,
    signer::Signer,
    system_instruction::transfer,
    system_program,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::{
    get_associated_token_address, get_associated_token_address_with_program_id,
};

use crate::{
    env::load_config,
    instruction::instruction::compute_ixs,
    liquidity::utils::tip_account,
    raydium_amm::swap::{instructions::SOLC_MINT, swapper::auth_keypair},
};

use super::{
    dao_burned_interface::{
        initialize_ix_with_program_id, InitializeIxArgs, InitializeIxData, InitializeKeys,
    },
    daos_transaction::{DAOSTrade, DAOS_BURNED_PROGRAM, DAOS_PROGRAM},
    fee_share_interface::FEE_SHARED,
    inx_builder::FUND_RAISE_PROGRAM,
};

pub async fn create_daos_fund() -> eyre::Result<()> {
    let args = load_config().await?;

    let wallet = Keypair::from_base58_string(&args.engine.payer_keypair);

    let token_mint = Keypair::new();

    let token_2022 = Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").unwrap();

    // let data = hex::decode("afaf6d1f0d989bedd85895a993e1139026fc516d4bd117680e0cf8863698a3a909feb8dd2f52c5d3d85895a993e1139026fc516d4bd117680e0cf8863698a3a909feb8dd2f52c5d332e71da0068df4413320771e15f5c6c914be93be28c31c007059c1dc39b97b3b").unwrap();

    // let decode = super::fee_share_interface::InitializeIxData::deserialize(&data);

    // println!("{decode:#?}");

    let depositor = Pubkey::find_program_address(
        &[b"state".as_ref(), token_mint.pubkey().as_ref()],
        &DAOS_BURNED_PROGRAM,
    )
    .0;

    let (init_wallet, _) = Pubkey::find_program_address(
        &[b"fee_share_state", token_mint.pubkey().as_ref()],
        &FEE_SHARED,
    );

    let init_wallet_keys = super::fee_share_interface::InitializeKeys {
        payer: wallet.pubkey(),
        dao_mint: token_mint.pubkey(),
        funding_mint: SOLC_MINT,
        fee_share_state: init_wallet,
        system_program: system_program::id(),
    };

    let initi_wallet_args = super::fee_share_interface::InitializeIxArgs {
        creator: wallet.pubkey(),
        referrer: wallet.pubkey(),
        platform: wallet.pubkey(),
    };

    let initialize_initi_wallet = super::fee_share_interface::initialize_ix_with_program_id(
        FEE_SHARED,
        init_wallet_keys,
        initi_wallet_args,
    )?;

    let (curve_pda, _) =
        Pubkey::find_program_address(&[b"curve", depositor.as_ref()], &DAOS_PROGRAM);

    let (fundraise_state, _) = Pubkey::find_program_address(
        &[
            b"fundraise",
            depositor.as_ref(),
            token_mint.pubkey().as_ref(),
        ],
        &FUND_RAISE_PROGRAM,
    );

    let token_vault =
        get_associated_token_address_with_program_id(&depositor, &token_mint.pubkey(), &token_2022);

    let (init_wallet, _) = Pubkey::find_program_address(
        &[b"wallet", token_mint.pubkey().as_ref()],
        &DAOS_BURNED_PROGRAM,
    );

    let keys = InitializeKeys {
        admin: wallet.pubkey(),
        state: depositor,
        wallet: init_wallet,
        dao_mint: token_mint.pubkey(),
        funding_mint: SOLC_MINT,
        dao_mint_vault: token_vault,
        token_program: token_2022,
        system_program: system_program::id(),
        associated_token_program: spl_associated_token_account::id(),
        fundraise_state,
        fundraise_program: FUND_RAISE_PROGRAM,
        fundraise_token_vault: get_associated_token_address_with_program_id(
            &fundraise_state,
            &token_mint.pubkey(),
            &token_2022,
        ),
    };

    let token_args = InitializeIxArgs {
        name: "JPIG".to_string(),
        symbol: "$JPIG".into(),
        uri: "https://ipfs.io/ipfs/QmPSdeyWGaAsGKEY3FKQmHVMoFXZtbMwrj59c6AcFMjqrD".into(),

        dao_duration_seconds: 7776000 as u32,
        funding_goal: 42000000000,
        funding_duration_seconds: 604800,
        carry_basis: Some(5000),
        fee_authority: Pubkey::from_str("D5bBVBQDNDzroQpduEJasYL5HkvARD6TcNu3yJaeVK5W").unwrap(),
    };

    let initialize = initialize_ix_with_program_id(DAOS_BURNED_PROGRAM, keys, token_args)?;
    let compute = compute_ixs(sol_to_lamports(args.trading.priority_fee), 200000)?;

    let mut instructions = vec![];
    instructions.extend(compute);
    instructions.push(initialize_initi_wallet);
    instructions.push(initialize);

    let rpc_client = RpcClient::new(args.network.rpc_url);

    let config = CommitmentLevel::Processed;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &wallet.pubkey(),
        &instructions,
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
        &[&wallet, &token_mint],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    if args.engine.use_bundles {
        let mut searcher_client =
            get_searcher_client(&args.network.block_engine_url, &Arc::new(auth_keypair())).await?;

        let tip_ix = transfer(
            &wallet.pubkey(),
            &tip_account(),
            sol_to_lamports(args.trading.bundle_tip),
        );
        instructions.push(tip_ix);

        let message = match solana_program::message::v0::Message::try_compile(
            &wallet.pubkey(),
            &instructions,
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
            &[&wallet, &token_mint],
        ) {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {:?}", e);
                return Ok(());
            }
        };

        let bundle_txn = vec![transaction];

        let mut bundle_results_subscription = searcher_client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        match send_bundle_with_confirmation(
            &bundle_txn,
            &Arc::new(rpc_client),
            &mut searcher_client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                error!("Error: {}", e);
            }
        };

        std::mem::drop(bundle_results_subscription);
    } else {
        info!("Sending Transaction");
        let config = RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        };

        if args.trading.spam {
            let mut counter = 0;
            while counter < args.trading.spam_count {
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

            rpc_client
                .confirm_transaction_with_spinner(
                    &result,
                    &latest_blockhash,
                    CommitmentConfig::confirmed(),
                )
                .await?;

            info!("Transaction Sent {:?}", result);
        }
    }
    Ok(())
}
