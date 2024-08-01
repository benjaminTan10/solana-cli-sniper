use std::{fs::File, io::Write};

use solana_address_lookup_table_program::instruction::create_lookup_table;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::{env::minter::PoolDataSettings, rpc::HTTP_CLIENT};

pub async fn create_lut(mut pool_data: PoolDataSettings) -> eyre::Result<(Instruction, Pubkey)> {
    println!("Creating LUT");
    let buyer_key = Keypair::from_base58_string(&pool_data.buyer_key);

    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let recent_blockhash = match rpc_client.get_latest_blockhash().await {
        Ok(blockhash) => blockhash,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e.into());
        }
    };
    let recent_slot = match rpc_client.get_slot().await {
        Ok(slot) => slot,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e.into());
        }
    };

    let (lut, lut_key) = create_lookup_table(buyer_key.pubkey(), buyer_key.pubkey(), recent_slot);

    pool_data.lut_key = lut_key.to_string();
    let mut file = File::create("bundler_settings.json")?;
    file.write_all(serde_json::to_string(&pool_data)?.as_bytes())?;
    // let transaction = Transaction::new_signed_with_payer(
    //     &[lut],
    //     Some(&buyer_key.pubkey()),
    //     &[&buyer_key],
    //     recent_blockhash,
    // );

    // match rpc_client.send_and_confirm_transaction(&transaction).await {
    //     Ok(_) => {}
    //     Err(e) => {
    //         eprintln!("Lut Error: {:?}", e);
    //         return Err(e.into());
    //     }
    // }

    Ok((lut, lut_key))
}
