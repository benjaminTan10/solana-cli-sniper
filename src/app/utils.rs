pub async fn settings_fetcher() -> eyre::Result<(Settings, HashMap<String, LocalWallet>)> {
    let mut file = File::open("settings.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let settings: Settings = serde_json::from_str(&contents)?;

    let mut wallet_secret_keys = HashMap::new();

    for (wallet, address) in settings.wallets.iter() {
        let wallet_private_key_bytes = match decode(address) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(eyre::eyre!(
                    "Failed to decode WALLET_PRIVATE_KEY for {}",
                    wallet
                ))
            }
        };

        let wallet_secret_key = SecretKey::from_slice(&wallet_private_key_bytes)
            .map_err(|e| {
                Box::new(CustomError(format!(
                    "Failed to create SecretKey from WALLET_PRIVATE_KEY: {}",
                    e
                ))) as Box<dyn std::error::Error + Send>
            })
            .unwrap();

        wallet_secret_keys.insert(wallet.clone(), LocalWallet::from(wallet_secret_key));
    }

    Ok((settings, wallet_secret_keys))
}
