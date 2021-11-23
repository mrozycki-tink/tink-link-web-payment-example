use rocket::http::Status;

#[macro_use]
extern crate rocket;

mod tink;

#[post("/payment-request/<market>/<currency>/<amount>")]
fn payment_request(market: &str, currency: &str, amount: u32) -> Result<String, Status> {
    let access_token = match tink::get_access_token() {
        Ok(token) => token,
        Err(_e) => return Err(Status::InternalServerError),
    };

    match tink::create_payment_request(&access_token, &market, &currency, amount) {
        Ok(()) => Ok(format!(
            "payment-request: {}; {} {}",
            market, amount, currency
        )),
        Err(_e) => Err(Status::InternalServerError),
    }
}

#[get("/payment-confirmation/<request_id>")]
fn payment_confirmation(request_id: &str) -> Result<String, Status> {
    let access_token = match tink::get_access_token() {
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
    rocket::build().mount("/", routes![payment_request, payment_confirmation])
}
