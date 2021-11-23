use std::fmt;

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
pub struct PaymentRequest {
    pub id: Option<String>,
    pub amount: u32,
    pub currency: String,
    pub market: String,
    pub source_message: String,
    pub recipient_name: String,
    pub remittance_information: RemittanceInformation,
    pub destinations: Vec<Destination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemittanceInformation {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Destination {
    pub account_number: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

pub struct TinkApiGateway {
    api_url: String,
    client_id: String,
    client_secret: String,
    reqwest_client: reqwest::Client,
}

impl TinkApiGateway {
    pub fn new(api_url: String, client_id: String, client_secret: String) -> Self {
        Self {
            api_url,
            client_id,
            client_secret,
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub async fn get_access_token(&self) -> Result<String, TinkApiError> {
        const SCOPES: &str = "payment:read,payment:write";
        let response = self
            .reqwest_client
            .post(format!("{}/api/v1/oauth/token", self.api_url))
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded; charset=utf-8",
            )
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
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
        &self,
        access_token: &str,
        market: &str,
        currency: &str,
        amount: u32,
    ) -> Result<PaymentRequest, TinkApiError> {
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

        Ok(self
            .reqwest_client
            .post(format!("{}/api/v1/payments/requests", self.api_url))
            .header("Content-Type", "application/json; charset=utf-8")
            .bearer_auth(access_token)
            .json(&payment_request)
            .send()
            .await?
            .json::<PaymentRequest>()
            .await?)
    }

    pub async fn get_transfer_status(
        &self,
        access_token: &str,
        request_id: &str,
    ) -> Result<PaymentRequest, TinkApiError> {
        Ok(self
            .reqwest_client
            .get(format!(
                "{}/api/v1/payments/requests/{}",
                self.api_url, request_id
            ))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<PaymentRequest>()
            .await?)
    }
}
