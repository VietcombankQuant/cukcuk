mod model;

use crate::model::{Branch, LoginParam, LoginResponse, ServiceResult};

const API_DOMAIN: &str = "graphapi.cukcuk.vn";

#[derive(serde::Deserialize)]
struct Foo {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_client = reqwest::Client::new();

    // Get access token
    let login_url = format!("https://{}/api/Account/Login", API_DOMAIN);
    let login_param = LoginParam::new(
        "bownb",
        "CUKCUKOpenPlatform",
        "cd76e8fe462e8af36d137cd78a522c2d83bbd0861f2905e9b3180165c0d53df0",
    );

    let request = api_client.post(&login_url).json(&login_param).build()?;
    println!("request {:?}", request.body().unwrap());

    let resp = api_client.execute(request).await?;
    let message = resp.text().await?;
    let result: ServiceResult<LoginResponse> = serde_json::from_str(&message)?;
    let access_token = match result.data() {
        None => "",
        Some(data) => data.token(),
    };
    println!("access_token {}", access_token);

    let authorization = format!("Bearer {}", access_token);

    // Get branches
    let branch_url = format!("https://{}/api/v1/branchs/all", API_DOMAIN);
    let resp = api_client
        .get(branch_url)
        .header("CompanyCode", "bownb")
        .header("Authorization", authorization)
        .send()
        .await?;

    let message = resp.text().await?;
    let results: ServiceResult<Vec<Branch>> = serde_json::from_str(&message)?;

    println!("Branches: {:#?}", results);

    Ok(())
}
