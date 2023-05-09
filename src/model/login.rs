use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
struct LoginData {
    #[serde(rename = "AppID")]
    app_id: String,
    domain: String,
    login_time: String,
}

impl LoginData {
    fn new(domain: &str, app_id: &str) -> Self {
        let login_time = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();

        Self {
            domain: domain.to_string(),
            app_id: app_id.to_string(),
            login_time,
        }
    }

    fn signature(&self, secret_key: &str) -> String {
        use hmac::digest::Mac;
        type HmacSha256 = hmac::SimpleHmac<sha2::Sha256>;

        let message_to_sign = serde_json::to_string(&self).unwrap_or(Default::default());

        let mut mac = HmacSha256::new(secret_key.as_bytes().into());
        mac.update(message_to_sign.as_bytes());

        let signature = mac.finalize().into_bytes();
        let signature = format!("{:x}", signature);
        signature
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct LoginParam {
    #[serde(rename = "AppID")]
    app_id: String,
    domain: String,
    login_time: String,
    signature_info: String,
}

impl LoginParam {
    pub fn new(domain: &str, app_id: &str, secret_key: &str) -> Self {
        let login_data = LoginData::new(domain, app_id);
        let signature = login_data.signature(secret_key);

        Self {
            domain: login_data.domain,
            app_id: login_data.app_id,
            login_time: login_data.login_time,
            signature_info: signature,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct LoginResponse {
    #[serde(rename = "AppID")]
    app_id: String,
    domain: String,
    access_token: String,
    company_code: String,
    environment: Option<String>,
}

impl LoginResponse {
    pub fn token(&self) -> &str {
        &self.access_token
    }
}
