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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PaymentRequest {
    id: Option<String>,
    amount: u32,
    currency: String,
    market: String,
    source_message: String,
    recipient_name: String,
    remittance_information: RemittanceInformation,
    destinations: Vec<Destination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemittanceInformation {
    #[serde(rename = "type")]
    type_field: String,
    value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Destination {
    account_number: String,
    #[serde(rename = "type")]
    type_field: String,
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

pub async fn create_payment_request(
    access_token: &str,
    market: &str,
    currency: &str,
    amount: u32,
) -> Result<String, TinkApiError> {
    let payment_request = PaymentRequest {
        id: None,
        amount,
        currency: currency.to_owned(),
        market: market.to_owned(),
        source_message: "Payment for Sneaker 034".to_owned(),
        recipient_name: "Demo Store AB".to_owned(),
        remittance_information: RemittanceInformation {
            type_field: "UNSTRUCTURED".to_owned(),
            value: "3245928392092".to_owned(),
        },
        destinations: vec![Destination {
            account_number: "33008808080808".to_owned(),
            type_field: "se".to_owned(),
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.tink.com/api/v1/payments/requests")
        .header("Content-Type", "application/json; charset=utf-8")
        .bearer_auth(access_token)
        .json(&payment_request)
        .send()
        .await?
        .json::<PaymentRequest>()
        .await?;

    Ok(response.id.unwrap())
}

pub fn get_transfer_status(_access_token: &str, _request_id: &str) -> Result<String, TinkApiError> {
    Ok("PENDING".to_owned())
}
