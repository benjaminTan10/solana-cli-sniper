use log::info;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct License {
    pub success: bool,
    pub error: String,
}

pub fn embed() -> String {
    let string = format!(
        r"

        ███╗   ███╗███████╗██╗   ██╗ █████╗ ██████╗ ██╗██╗  ██╗
        ████╗ ████║██╔════╝██║   ██║██╔══██╗██╔══██╗██║██║ ██╔╝
        ██╔████╔██║█████╗  ██║   ██║███████║██████╔╝██║█████╔╝
        ██║╚██╔╝██║██╔══╝  ╚██╗ ██╔╝██╔══██║██╔══██╗██║██╔═██╗
        ██║ ╚═╝ ██║███████╗ ╚████╔╝ ██║  ██║██║  ██║██║██║  ██╗
        ╚═╝     ╚═╝╚══════╝  ╚═══╝  ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝
                "
    );

    string
}

pub async fn license_checker() -> Result<(), Box<dyn std::error::Error>> {
    info!("Verifying License Key...");

    //reqwest to the server
    let client = reqwest::Client::new();
    let res = client
        .post("https://log.mal.rocks/login")
        .header("Content-Type", "application/json")
        .header("User-Agent", "insomnia/8.6.0")
        .json(&json!({"key": "test"}))
        .send()
        .await
        .unwrap();

    let response_text = res.text().await?;
    let response: License = serde_json::from_str(&response_text)?;

    if response.success == true {
        info!("Successfully Logged In...");
    } else {
        let e = "Failed to verify license, closing in 10 seconds, think this is a error join the discord: discord.gg/foo";
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e).into());
    }

    Ok(())
}
