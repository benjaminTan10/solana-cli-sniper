use demand::{Confirm, DemandOption, Select};
use std::{error::Error, fs};

use crate::pumpfun::bundler::menu::pump_bundler;

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct JsonMetaData {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
    pub showName: bool,
    pub createdOn: String,
    pub twitter: String,
    pub telegram: String,
    pub website: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Creator {
    pub name: String,
    pub site: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ImageContainer {
    pub image: Vec<u8>,
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct IpFsPinata {
    pub IpfsHash: String,
    pub PinSize: u64,
    pub Timestamp: String,
    pub isDuplicate: Option<bool>,
}

pub const PINATA_API: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiJjNDc2YzEwMC00ZDViLTQ3YzQtYmNlNy0wOTQ0MDc3ODM3NDMiLCJlbWFpbCI6InRhaW1vb3JzaGFmaXF1ZTU0MUBnbWFpbC5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwicGluX3BvbGljeSI6eyJyZWdpb25zIjpbeyJpZCI6IkZSQTEiLCJkZXNpcmVkUmVwbGljYXRpb25Db3VudCI6MX0seyJpZCI6Ik5ZQzEiLCJkZXNpcmVkUmVwbGljYXRpb25Db3VudCI6MX1dLCJ2ZXJzaW9uIjoxfSwibWZhX2VuYWJsZWQiOmZhbHNlLCJzdGF0dXMiOiJBQ1RJVkUifSwiYXV0aGVudGljYXRpb25UeXBlIjoic2NvcGVkS2V5Iiwic2NvcGVkS2V5S2V5IjoiNDI3Yzc1MWFjYjU2MjYzZTczZjEiLCJzY29wZWRLZXlTZWNyZXQiOiIzYWQyMGFiZTQ3Y2U3Njg1MzhmMTNlOGQ4ZWFlZGRjMzEzZjlmMTY5YTI3ZTUxZjA5YThjZDJmMzJiYmVkMjEwIiwiaWF0IjoxNzIwNTAwMzgxfQ.k5RhP_yLlxWbMg63NXaUxeCJ2TdVyOOq-y2y_fPlNqM";

pub async fn check() -> Result<bool, Box<dyn std::error::Error>> {
    let confirm = Confirm::new("Is the Mint Metadata Correct?")
        .description(
            "Agreeing to this will upload the metadata to IPFS and generate a link. Are you sure you want to proceed?",
        )
        .affirmative("No")
        .negative("Yes")
        .selected(false)
        .run()
        .unwrap();

    Ok(!confirm)
}

pub async fn list_images() -> Result<Vec<u8>, Box<dyn Error>> {
    let paths = fs::read_dir(".")?;

    let mut image_files = Vec::new();
    for path in paths {
        let path = path?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("png") {
            image_files.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }

    let mut select = Select::new(" Image")
        .description("Select the Image ")
        .filterable(true);

    for image_file in &image_files {
        select = select.option(DemandOption::new(image_file).label(image_file));
    }

    let selected_option = select.run()?;

    //convert image into blob
    let image = fs::read(selected_option)?;

    Ok(image)
}

pub async fn image_upload() -> Result<String, Box<dyn std::error::Error>> {
    let image = list_images().await?;
    let boundary = "--------------------------970379464229125510173661";
    let mut payload = format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"file.png\"\r\nContent-Type: image/png\r\n\r\n",
    )
    .into_bytes();

    payload.extend(image.clone());
    payload.extend(format!("\r\n--{boundary}--\r\n").into_bytes());

    let client = reqwest::Client::new();

    let res = client
        .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", boundary),
        )
        .header("Authorization", format!("Bearer {}", PINATA_API))
        .body(payload)
        .send()
        .await?;

    let body = res.text().await?;

    let body: IpFsPinata = serde_json::from_str(&body)?;

    println!("Image Uploaded: {:?}", body.IpfsHash);

    Ok(body.IpfsHash)
}

pub async fn metadata_json() -> Result<JsonMetaData, Box<dyn std::error::Error>> {
    let bundle = fs::read_to_string("metadata.json")?;
    let mut bundle: JsonMetaData = serde_json::from_str(&bundle)?;
    let image = image_upload().await?;

    bundle.image = format!("https://ipfs.io/ipfs/{}", image);

    let check = check().await?;

    println!("{:?}", bundle);

    if !check {
        pump_bundler().await?;
        return Ok(bundle);
    }

    let client = reqwest::Client::new();

    let res = client
        .post("https://api.pinata.cloud/pinning/pinJSONToIPFS")
        .header("Authorization", format!("Bearer {}", PINATA_API))
        .json(&bundle)
        .send()
        .await?;

    let body = res.text().await?;

    let body: IpFsPinata = serde_json::from_str(&body)?;

    println!("Success: {:?}", body);

    Ok(bundle)
}
