mod branch;
mod invoice;
mod login;

const API_DOMAIN: &str = "graphapi.cukcuk.vn";

#[derive(Debug)]
pub struct LoginSession {
    api_client: reqwest::Client,
}
