use std::fmt;

#[derive(Debug, Clone)]
pub struct TinkApiError;

impl fmt::Display for TinkApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tink API error")
    }
}

pub fn get_access_token() -> Result<String, TinkApiError> {
    Ok("12345".to_owned())
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
