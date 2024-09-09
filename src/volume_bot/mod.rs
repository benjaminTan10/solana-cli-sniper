use std::{
    fs::File,
    io::{Read, Write},
    str::FromStr,
    sync::Arc,
};

mod volume_buyer;

use crate::{
    app::theme,
    auth::auth_verification,
    env::{
        load_config,
        minter::{load_minter_settings, PoolDataSettings},
    },
    liquidity::{
        lut::create_lut::create_lut,
        option::{
            sol_distribution::{atas_creation, distributor},
            wallet_gen::gen_wallet_save,
            wrap_sol::sol_wrap,
        },
        utils::{tip_account, tip_txn},
    },
    raydium_amm::{
        pool_searcher::amm_keys::pool_keys_fetcher,
        subscribe::PoolKeysSniper,
        swap::{instructions::TAX_ACCOUNT, swapper::auth_keypair},
        volume_pinger::volume::generate_volume,
    },
    rpc::HTTP_CLIENT,
    user_inputs::tokens::token_env,
};
use async_recursion::async_recursion;
use demand::{DemandOption, Select};
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
use solana_address_lookup_table_program::instruction::extend_lookup_table;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};

use self::volume_buyer::volume_wallets_buyer;

#[async_recursion]
pub async fn volume_menu() -> eyre::Result<()> {
    let _auth = match auth_verification().await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let theme = theme();
    let ms = Select::new("Volume Bot")
        .description("Select the Volume Bot")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Walletgen").label("▪ Generate Wallets"))
        // .option(DemandOption::new("LookupTable").label("[2] Create LUT"))
        .option(DemandOption::new("distributesol").label("▪ Distribute SOL"))
        // .option(DemandOption::new("Wrap SOL & ATAs").label("[3] Wrap SOL & ATAs"))
        .option(DemandOption::new("Volume").label("▪ Volume (Instant Seller) - 1 Wallet"))
        .option(
            DemandOption::new("VolumeBuyer")
                .label("▪ Volume Buyer - Multi Wallet")
                .selected(true),
        )
        .option(DemandOption::new("VolumeSeller").label("▪ Volume Seller - Multi Wallet"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Walletgen" => {
            let _ = gen_wallet_save().await;
            println!("-------------------Returning to Main Menu-------------------");
            volume_menu().await?;
        }
        // "LookupTable" => {
        //     let _ = volume_lut().await;
        //     println!("-------------------Returning to Main Menu-------------------");
        //     volume_menu().await?;
        // }
        "distributesol" => {
            let _ = distributor().await;
            println!("-------------------Returning to Main Menu-------------------");
            volume_menu().await?;
        }
        // "Wrap SOL & ATAs" => {
        //     let _ = sol_wrap().await;
        //     println!("-------------------Returning to Main Menu-------------------");
        //     volume_menu().await?;
        // }
        "Volume" => {
            let _ = generate_volume().await;
            println!("-------------------Returning to Main Menu-------------------");
            volume_menu().await?;
        }
        "VolumeBuyer" => {
            let _ = volume_wallets_buyer(false).await;
        }
        "VolumeSeller" => {
            let _ = volume_wallets_buyer(true).await;
        }
        "Main Menu" => {
            let _ = crate::app::app(false).await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

pub async fn volume_lut() -> eyre::Result<()> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let mut data = load_minter_settings().await?;
    let engine = load_config().await?;

    let pool_id = token_env("Pool ID:").await;

    let buyer_key = Arc::new(Keypair::from_base58_string(&data.buyer_key));

    let mut lut_ix = vec![];

    let (ix, lut_key) = create_lut(data.clone()).await?;
    lut_ix.push(ix);

    // let amm_keys = load_amm_keys(&connection, &AMM_PROGRAM, &pool_id).await?;
    // let market_keys = load_pool_keys(pool_id, amm_keys).await?;

    let pool_keys = pool_keys_fetcher(pool_id).await?;

    let mut pool_ex_lut = poolkeys_lut_2(pool_keys, lut_key, data.clone())?;

    let (_, mint_ata, sol_ata) = atas_creation(
        vec![buyer_key.pubkey()],
        buyer_key.clone(),
        Pubkey::from_str(&data.token_mint)?,
    )
    .await?;

    pool_ex_lut.accounts.push(AccountMeta::new(mint_ata, false));
    pool_ex_lut.accounts.push(AccountMeta::new(sol_ata, false));

    lut_ix.push(pool_ex_lut);

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let tax = tip_txn(buyer_key.pubkey(), TAX_ACCOUNT, sol_to_lamports(0.1));
    let tip = tip_txn(buyer_key.pubkey(), tip_account(), sol_to_lamports(0.001));

    lut_ix.push(tax);
    lut_ix.push(tip);

    info!("Building transaction...");

    let mut txns = vec![];

    for ix in &lut_ix {
        let versioned_msg = VersionedMessage::V0(Message::try_compile(
            &buyer_key.pubkey(),
            &[ix.clone()],
            &[],
            recent_blockhash,
        )?);

        let transaction = VersionedTransaction::try_new(versioned_msg, &[&buyer_key])?;

        txns.push(transaction);
    }

    let mut search =
        get_searcher_client(&engine.network.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = search
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    info!("Subscribed to bundle results");

    let bundle = match send_bundle_with_confirmation(
        &txns,
        &connection,
        &mut search,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(_) => {
            data.volume_lut_key = lut_key.to_string();
            let mut file = File::create("bundler_settings.json")?;
            file.write_all(serde_json::to_string(&data)?.as_bytes())?;
        }
        Err(e) => {
            eprintln!("Distribution Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    Ok(())
}

pub fn poolkeys_lut_2(
    pool_keys: PoolKeysSniper,
    lut: Pubkey,
    server_data: PoolDataSettings,
) -> eyre::Result<Instruction> {
    let buyer_wallet = Keypair::from_base58_string(&server_data.buyer_key);

    let mut keys = vec![
        pool_keys.id,
        pool_keys.base_mint,
        pool_keys.quote_mint,
        pool_keys.lp_mint,
        pool_keys.program_id,
        pool_keys.authority,
        pool_keys.open_orders,
        pool_keys.target_orders,
        pool_keys.base_vault,
        pool_keys.quote_vault,
        pool_keys.withdraw_queue,
        pool_keys.lp_vault,
        pool_keys.market_program_id,
        pool_keys.market_id,
        pool_keys.market_authority,
        pool_keys.market_base_vault,
        pool_keys.market_quote_vault,
        pool_keys.market_bids,
        pool_keys.market_asks,
        pool_keys.market_event_queue,
        pool_keys.lookup_table_account,
    ];

    let add_accounts = extend_lookup_table(
        lut,
        buyer_wallet.pubkey(),
        Some(buyer_wallet.pubkey()),
        keys,
    );

    Ok(add_accounts)
}
