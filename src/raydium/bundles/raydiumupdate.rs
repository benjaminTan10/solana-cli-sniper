use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use log::{error, info};
use serde::Deserialize;
use solana_program::pubkey;
use solana_sdk::pubkey::Pubkey;
use std::{str::FromStr, time::Instant};
use tokio::io::AsyncWriteExt;

use super::mev_trades::POOL_KEYS;
use crate::raydium::subscribe::PoolKeysSniper;
use console::{style, Emoji};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub async fn update_raydium() -> Result<Vec<Pubkey>, Box<dyn std::error::Error>> {
    let url = "https://api.raydium.io/v2/sdk/liquidity/mainnet.json";
    let spinner_style = ProgressStyle::default_spinner()
        .template("{prefix:.bold.dim} {spinner} {wide_msg}")?
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    println!(
        "{} {}Resolving packages...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );
    println!(
        "{} {}Fetching packages...",
        style("[2/4]").bold().dim(),
        TRUCK
    );

    let client = reqwest::Client::new();
    let total_size = {
        let resp = reqwest::get(url).await?;
        let headers = resp.headers();
        let total_size = headers
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse().ok())
            .unwrap_or(0);
        total_size
    };

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));

    let mut request = client.get(url).build()?;
    request.headers_mut().insert(
        reqwest::header::RANGE,
        format!("bytes={}-", pb.position()).parse().unwrap(),
    );

    let started = Instant::now();
    let mut source = client.execute(request).await?;
    let mut dest = tokio::fs::File::create("mainnet.json").await?;
    while let Some(chunk) = source.chunk().await? {
        dest.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message("download completed");

    println!(
        "{} {}Linking dependencies...",
        style("[3/4]").bold().dim(),
        CLIP
    );
    println!(
        "{} {}Building fresh packages...",
        style("[4/4]").bold().dim(),
        PAPER
    );

    let json = tokio::fs::read_to_string("mainnet.json")
        .await
        .expect("Error reading file");
    println!("Length of JSON: {}", json.len());
    let (_, keys) = match load_json_to_hashmap(&json).await {
        Ok((b, keys, errors)) => {
            info!("Loaded pools: {}, FrontRun Tokens {}", b, keys.len());
            info!("Parse Errors: {}", errors);
            (b, keys)
        }
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    Ok(keys)
}

pub async fn load_json_to_hashmap(
    json: &str,
) -> Result<(bool, Vec<Pubkey>, usize), Box<dyn std::error::Error>> {
    let json = json.to_owned();
    let data: Pools = tokio::task::spawn_blocking(move || serde_json::from_str(&json)).await??;
    println!("Number of pools in JSON: {}", data.official.len());
    let mut map = POOL_KEYS.lock().unwrap();
    let mut ids = Vec::new();
    let mut parse_errors = 0;
    for pool in &data.official {
        match Pubkey::from_str(&pool.id) {
            Ok(pubkey) => {
                map.insert(pool.id.clone(), pool.clone());
                ids.push(pubkey);
            }
            Err(_) => {
                parse_errors += 1;
            }
        }
    }
    info!("Fetched keys for account: {:?}", map.len());
    Ok((data.official.len() == map.len(), ids, parse_errors))
}
#[derive(Debug, Deserialize)]
pub struct Pools {
    name: String,
    official: Vec<PoolKeysSniper>,
}
