use rocket::http::Status;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

mod tink;

#[post("/payment-request/<market>/<currency>/<amount>")]
async fn payment_request(market: &str, currency: &str, amount: u32) -> Result<String, Status> {
    let access_token = match tink::get_access_token().await {
        Ok(token) => token,
        Err(_e) => return Err(Status::InternalServerError),
    };

    match tink::create_payment_request(&access_token, &market, &currency, amount).await {
        Ok(id) => Ok(format!("payment request id: {}", id)),
        Err(_e) => Err(Status::InternalServerError),
    }
}

#[get("/payment-confirmation/<request_id>")]
async fn payment_confirmation(request_id: &str) -> Result<String, Status> {
    let access_token = match tink::get_access_token().await {
        Ok(token) => token,
        Err(_e) => return Err(Status::InternalServerError),
    };

    match tink::get_transfer_status(&access_token, &request_id) {
        Ok(status) => Ok(format!(
            "payment-confirmation: {}; status: {}",
            request_id, status
        )),
        Err(_e) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    if std::env::var("REACT_APP_TINK_LINK_PAYMENT_CLIENT_ID").is_err() {
        panic!("REACT_APP_TINK_LINK_PAYMENT_CLIENT_ID env var needs to be set");
    }
    if std::env::var("TINK_LINK_PAYMENT_CLIENT_SECRET").is_err() {
        panic!("TINK_LINK_PAYMENT_CLIENT_SECRET env var needs to be set");
    }
    rocket::build().mount("/", routes![payment_request, payment_confirmation])
}
