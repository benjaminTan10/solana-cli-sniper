use std::time::Duration;

use jito_sdk_rust::JitoJsonRpcSDK;
use log::info;
use serde_json::Value;

/// Monitors the status of a submitted bundle
pub async fn monitor_bundle_status(
    jito_sdk: &JitoJsonRpcSDK,
    bundle_uuid: &str,
    max_retries: u32,
    retry_delay: Duration,
) -> anyhow::Result<()> {
    for attempt in 1..=max_retries {
        info!(
            "Checking bundle status (attempt {}/{})",
            attempt, max_retries
        );

        let status_response = jito_sdk
            .get_in_flight_bundle_statuses(vec![bundle_uuid.to_string()])
            .await?;

        if let Some(status) = extract_bundle_status(&status_response) {
            match status {
                "Landed" => {
                    info!("Bundle landed on-chain!");
                    return Ok(());
                }
                "Pending" => {
                    info!("Bundle is pending. Waiting...");
                }
                status => {
                    info!("Unexpected bundle status: {}. Waiting...", status);
                }
            }
        } else {
            info!("Unable to parse bundle status. Waiting...");
        }

        if attempt < max_retries {
            tokio::time::sleep(retry_delay).await;
        }
    }

    Err(anyhow::anyhow!(
        "Failed to confirm bundle status after {} attempts",
        max_retries
    ))
}

// Helper function to extract bundle status from response
fn extract_bundle_status(response: &Value) -> Option<&str> {
    response
        .get("result")?
        .get("value")?
        .as_array()?
        .get(0)?
        .get("status")?
        .as_str()
}
