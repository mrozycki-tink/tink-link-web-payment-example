use rocket::{http::Status, State};
use tink::TinkApiGateway;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

mod tink;

#[post("/payment-request/<market>/<currency>/<amount>")]
async fn payment_request(
    market: &str,
    currency: &str,
    amount: u32,
    tink: &State<TinkApiGateway>,
) -> Result<String, Status> {
    let access_token = match tink.get_access_token().await {
        Ok(token) => token,
        Err(_e) => return Err(Status::InternalServerError),
    };

    match tink
        .create_payment_request(&access_token, market, currency, amount)
        .await
    {
        Ok(payment_request) => Ok(format!("payment request id: {:?}", payment_request.id)),
        Err(_e) => Err(Status::InternalServerError),
    }
}

#[get("/payment-confirmation/<request_id>")]
async fn payment_confirmation(
    request_id: &str,
    tink: &State<TinkApiGateway>,
) -> Result<String, Status> {
    let access_token = match tink.get_access_token().await {
        Ok(token) => token,
        Err(_e) => return Err(Status::InternalServerError),
    };

    match tink.get_transfer_status(&access_token, request_id).await {
        Ok(request) => Ok(format!("Payment: {:?}", request)),
        Err(_e) => Err(Status::InternalServerError),
    }
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
