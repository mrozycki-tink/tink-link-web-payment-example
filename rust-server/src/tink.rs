use std::{env, fmt};

#[derive(Debug, Clone)]
pub struct TinkApiError;

impl fmt::Display for TinkApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tink API error")
    }
}

impl<T> From<T> for TinkApiError
where
    T: std::error::Error,
{
    fn from(_: T) -> Self {
        Self {}
    }
}

#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
    token_type: String,
    expires_in: u32,
    access_token: String,
    scope: String,
}

pub async fn get_access_token() -> Result<String, TinkApiError> {
    const SCOPES: &str = "payment:read,payment:write";
    let client_id = env::var("REACT_APP_TINK_LINK_PAYMENT_CLIENT_ID").unwrap();
    let client_secret = env::var("TINK_LINK_PAYMENT_CLIENT_SECRET").unwrap();

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.tink.com/api/v1/oauth/token")
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=utf-8",
        )
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "client_credentials"),
            ("scope", SCOPES),
        ])
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;

    Ok(response.access_token)
}

pub fn create_payment_request(
    _access_token: &str,
    _market: &str,
    _currency: &str,
    _amount: u32,
) -> Result<(), TinkApiError> {
    Ok(())
}

pub fn get_transfer_status(_access_token: &str, _request_id: &str) -> Result<String, TinkApiError> {
    Ok("PENDING".to_owned())
}
