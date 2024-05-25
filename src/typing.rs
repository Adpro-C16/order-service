use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

use crate::guard::services::TransactionType;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub status_code: Status,
    pub message: &'static str,
}

pub fn error_response(status_code: Status, message: &'static str) -> Custom<Json<ErrorResponse>> {
    return Custom(
        status_code,
        Json::from(ErrorResponse {
            status_code: status_code,
            message: message,
        }),
    );
}

pub type Error = Custom<Json<ErrorResponse>>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl TransactionType {
    pub fn to_i32(&self) -> i32 {
        match self {
            TransactionType::Deposit => 0,
            TransactionType::Withdraw => 1,
        }
    }
}
