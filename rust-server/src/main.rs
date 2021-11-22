mod tink;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};

#[post("/payment-request/{market}/{currency}/{amount}")]
async fn payment_request(
    web::Path((market, currency, amount)): web::Path<(String, String, u32)>,
) -> HttpResponse {
    let access_token = match tink::get_access_token() {
        Ok(token) => token,
        Err(e) => return HttpResponse::InternalServerError().body(format!("{}", e)),
    };

    match tink::create_payment_request(&access_token, &market, &currency, amount) {
        Ok(()) => HttpResponse::Ok().body(format!(
            "payment-request: {}; {} {}",
            market, amount, currency
        )),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

#[get("/payment-confirmation/{request_id}")]
async fn payment_confirmation(web::Path(request_id): web::Path<String>) -> HttpResponse {
    let access_token = match tink::get_access_token() {
        Ok(token) => token,
        Err(e) => return HttpResponse::InternalServerError().body(format!("{}", e)),
    };

    match tink::get_transfer_status(&access_token, &request_id) {
        Ok(status) => HttpResponse::Ok().body(format!(
            "payment-confirmation: {}; status: {}",
            request_id, status
        )),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(payment_request)
            .service(payment_confirmation)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
