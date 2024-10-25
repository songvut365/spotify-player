use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

pub fn new_error_response<T: std::fmt::Display>(err: T) -> String {
    let error_response = ErrorResponse {
        message: err.to_string(),
    };

    let error_response_json = match serde_json::to_string(&error_response) {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    error_response_json
}
