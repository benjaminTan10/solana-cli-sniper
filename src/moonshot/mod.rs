use std::{str::FromStr, sync::Arc};

use instructions::accounts::CurveAccountAccount;
use solana_program::pubkey;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    moonshot::{self, instructions::instructions::MOONSHOT_TOKEN_LAUNCHPAD},
    raydium_cpmm::instructions::rpc,
    rpc::HTTP_CLIENT,
};

pub mod instructions;
pub mod menu;
pub mod sniper;
pub mod swap;

pub const BACKEND_AUTHORITY: Pubkey = pubkey!("Cb8Fnhp95f9dLxB3sYkNCbN3Mjxuc3v2uQZ7uVeqvNGB");

fn get_moonshot_curve(mint: Pubkey, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"curve-account", &mint.to_bytes()], program_id).0
}

pub async fn generate_moonshot_buy_ix(
    token: Pubkey,
    token_amount: u64,
    main_signer: Arc<Keypair>,
) -> eyre::Result<()> {
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let account = &Pubkey::from_str("2aLqNYy4a3X3u5EgEJHiWGHYHuEKBbAHqx7HShX6MDYN").unwrap();
    let account_data = rpc_client.get_account_data(account).await.unwrap();

    let sliced_data: &mut &[u8] = &mut account_data.as_slice();

    let curve_account = CurveAccountAccount::deserialize(sliced_data).unwrap();

    println!("Curve Account: {:?}", curve_account);

    let bonding_curve_pda = get_moonshot_curve(token, &MOONSHOT_TOKEN_LAUNCHPAD);
    println!("Bonding Curve PDA: {:?}", bonding_curve_pda);
    let bonding_curve_ata = get_associated_token_address(&bonding_curve_pda, &token);
    let signer_ata = get_associated_token_address(&main_signer.pubkey(), &token);
    // let sell_ix = moonshot::instructions::instructions::buy_ix_with_program_id(
    //     MOONSHOT_TOKEN_LAUNCHPAD,
    //     moonshot::instructions::instructions::BuyKeys {
    //         sender: main_signer.pubkey(),
    //         backend_authority: BACKEND_AUTHORITY,
    //         sender_token_account: signer_ata,
    //         curve_account: bonding_curve_pda,
    //         associated_bonding_curve: bonding_curve_ata,
    //         associated_user: signer_ata,
    //         user: main_signer.pubkey(),
    //         system_program: system_program::id(),
    //         associated_token_program: spl_associated_token_account::id(),
    //         token_program: spl_token::id(),
    //         event_authority: EVENT_AUTH,
    //         program: PUMP_PROGRAM,
    //     },
    //     SellIxArgs {
    //         amount: token_amount,
    //         min_sol_output: 0,
    //     },
    // )?;

    Ok(())
}
