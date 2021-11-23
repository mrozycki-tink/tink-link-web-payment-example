use rocket::{http::Status, State};
use tink::{TinkApiError, TinkApiGateway};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

mod tink;

impl From<TinkApiError> for Status {
    fn from(e: TinkApiError) -> Self {
        eprintln!("Internal server error: {}", e);
        Status::InternalServerError
    }
}

#[post("/payment-request/<market>/<currency>/<amount>")]
async fn payment_request(
    market: &str,
    currency: &str,
    amount: u32,
    tink: &State<TinkApiGateway>,
) -> Result<serde_json::Value, Status> {
    let access_token = tink.get_access_token().await?;

    let payment_request = tink
        .create_payment_request(&access_token, market, currency, amount)
        .await?;

    Ok(json!({ "data": payment_request, "token": access_token }))
}

#[post("/payment-confirmation/<request_id>")]
async fn payment_confirmation(
    request_id: &str,
    tink: &State<TinkApiGateway>,
) -> Result<serde_json::Value, Status> {
    let access_token = tink.get_access_token().await?;
    let data = tink.get_transfer_status(&access_token, request_id).await?;

    Ok(json!({ "data": data }))
}

#[launch]
fn rocket() -> _ {
    let client_id = std::env::var("REACT_APP_TINK_LINK_PAYMENT_CLIENT_ID")
        .expect("REACT_APP_TINK_LINK_PAYMENT_CLIENT_ID env var needs to be set");

    let client_secret = std::env::var("TINK_LINK_PAYMENT_CLIENT_SECRET")
        .expect("TINK_LINK_PAYMENT_CLIENT_SECRET env var needs to be set");

    rocket::build()
        .manage(tink::TinkApiGateway::new(
            "https://api.tink.com".to_owned(),
            client_id,
            client_secret,
        ))
        .mount("/", routes![payment_request, payment_confirmation])
}
