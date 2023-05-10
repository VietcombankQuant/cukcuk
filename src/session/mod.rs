use reqwest::header::{HeaderMap, AUTHORIZATION};

use crate::model::{LoginParam, LoginResponse, ServiceResult};

mod branch;

const API_DOMAIN: &str = "graphapi.cukcuk.vn";

#[derive(Debug)]
pub struct LoginSession {
    api_client: reqwest::Client,
}

impl LoginSession {
    pub async fn new(login_param: &LoginParam) -> anyhow::Result<Self> {
        let login_url = format!("https://{}/api/Account/Login", API_DOMAIN);
        let api_client = reqwest::Client::new();
        let resp = api_client.post(&login_url).json(login_param).send().await?;

        let message = resp.text().await?;
        let result: ServiceResult<LoginResponse> = serde_json::from_str(&message)?;
        if !result.success {
            return Err(anyhow::anyhow!(
                "Failed to login with error code {} - error message {}",
                result.error_type.unwrap_or_default(),
                result.error_message.unwrap_or_default()
            ));
        }

        let access_token = result.data.unwrap_or_default().access_token;
        let api_client = LoginSession::api_client(&login_param.domain, &access_token)?;

        Ok(Self { api_client })
    }

    fn api_client(company_name: &str, access_token: &str) -> anyhow::Result<reqwest::Client> {
        let auth = format!("Bearer {}", access_token);
        let mut headers = HeaderMap::new();
        headers.insert("CompanyCode", company_name.parse()?);
        headers.insert(AUTHORIZATION, auth.parse()?);
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(client)
    }
}
