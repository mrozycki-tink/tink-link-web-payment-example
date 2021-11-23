use std::{collections::HashMap, env, fmt};

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
struct PaymentRequest<'a> {
    id: Option<&'a str>,
    amount: u32,
    currency: &'a str,
    market: &'a str,
    source_message: &'a str,
    recipient_name: &'a str,
    remittance_information: RemittanceInformation<'a>,
    destinations: Vec<Destination<'a>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemittanceInformation<'a> {
    #[serde(rename = "type")]
    type_field: &'a str,
    value: &'a str,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Destination<'a> {
    account_number: &'a str,
    #[serde(rename = "type")]
    type_field: &'a str,
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
        currency,
        market,
        source_message: "Payment for Sneaker 034",
        recipient_name: "Demo Store AB",
        remittance_information: RemittanceInformation {
            type_field: "UNSTRUCTURED",
            value: "3245928392092",
        },
        destinations: vec![Destination {
            account_number: "33008808080808",
            type_field: "se",
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
        .json::<HashMap<String, String>>()
        .await?;

    Ok(response.get("id").unwrap().to_owned())
}

pub fn get_transfer_status(_access_token: &str, _request_id: &str) -> Result<String, TinkApiError> {
    Ok("PENDING".to_owned())
}
