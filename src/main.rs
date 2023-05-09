use std::fmt::format;

use hmac::digest::{KeyInit, Mac, Update};
use serde::{Deserialize, Serialize};

const API_DOMAIN: &str = "graphapi.cukcuk.vn";

#[derive(Serialize, Deserialize, Debug, Default)]
struct LoginData {
    #[serde(rename = "Domain")]
    pub domain: String,

    #[serde(rename = "AddID")]
    pub app_id: String,

    #[serde(rename = "LoginTime")]
    pub login_time: String,
}

impl LoginData {
    fn new(domain: &str, app_id: &str) -> Self {
        let login_time = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();

        Self {
            domain: domain.to_string(),
            app_id: app_id.to_string(),
            login_time: login_time,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct LoginParam {
    #[serde(rename = "Domain")]
    pub domain: String,

    #[serde(rename = "AddID")]
    pub app_id: String,

    #[serde(rename = "LoginTime")]
    pub login_time: String,

    #[serde(rename = "SignatureInfo")]
    pub signature: String,
}

impl LoginParam {
    fn new(domain: &str, app_id: &str, secret_key: &str) -> Self {
        let login_data = LoginData::new(domain, app_id);
        let login_data_str = serde_json::to_string(&login_data).unwrap();

        type HmacSha256 = hmac::SimpleHmac<sha2::Sha256>;
        let mut mac = HmacSha256::new(secret_key.as_bytes());
        mac.update(login_data_str.as_bytes());
        let signature = mac.finalize().into_bytes();
        let signature = format!("{:x}", signature);

        Self {
            domain: login_data.domain,
            app_id: login_data.app_id,
            login_time: login_data.login_time,
            signature,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let login_url = format!("https://{}/api/Account/Login");
    let api_client = reqwest::Client::new();

    let login_param = LoginParam::new(
        "bownb",
        "CUKCUKOpenPlatform",
        "7e1612bffb97ce9bc4c5c47e5c972ca566c951e030ae828569533fec73ba41f5",
    );

    println!("Signature: {}", login_param.signature);

    //let resp = api_client.get(&login_url).json(&login_data).send().await?;

    Ok(())
}
