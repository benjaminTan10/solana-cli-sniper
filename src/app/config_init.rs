use eyre::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::env::{load_config, SettingsConfig};

// Define a struct to hold our global state
pub struct GlobalConfig {
    config: Option<SettingsConfig>,
    additional_data: HashMap<String, String>,
}

// Create a global instance of our state
static GLOBAL_CONFIG: Lazy<RwLock<GlobalConfig>> = Lazy::new(|| {
    RwLock::new(GlobalConfig {
        config: None,
        additional_data: HashMap::new(),
    })
});

// Function to initialize the global config
pub async fn initialize_global_config() -> Result<()> {
    let config = load_config().await?;
    let mut global_config = GLOBAL_CONFIG.write().await;
    global_config.config = Some(config);
    Ok(())
}

// Function to get a reference to the config
pub async fn get_config() -> Result<SettingsConfig> {
    let global_config = GLOBAL_CONFIG.read().await;
    global_config
        .config
        .clone()
        .ok_or_else(|| eyre::eyre!("Config not initialized"))
}

// Function to update a specific field in the config
pub async fn update_config_field<T: Clone>(
    update_fn: impl FnOnce(&mut SettingsConfig) -> &mut T,
    new_value: T,
) -> Result<()> {
    let mut global_config = GLOBAL_CONFIG.write().await;
    if let Some(config) = &mut global_config.config {
        *update_fn(config) = new_value;
        // Here you might want to save the updated config to file
        // fs::write("config.toml", toml::to_string_pretty(&config)?)?;
        Ok(())
    } else {
        Err(eyre::eyre!("Config not initialized"))
    }
}

// Function to add additional data to the global state
pub async fn set_additional_data(key: &str, value: String) {
    let mut global_config = GLOBAL_CONFIG.write().await;
    global_config.additional_data.insert(key.to_string(), value);
}

// Function to get additional data from the global state
pub async fn get_additional_data(key: &str) -> Option<String> {
    let global_config = GLOBAL_CONFIG.read().await;
    global_config.additional_data.get(key).cloned()
}
