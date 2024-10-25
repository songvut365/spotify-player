use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetTokenRequest {
    pub code: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetAccessTokenResponse {
    Success(GetAccessTokenResponseSuccess),
    Error(GetAccessTokenResponseError),
}

#[derive(Deserialize, Serialize)]
pub struct GetAccessTokenResponseSuccess {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i32,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetAccessTokenResponseError {
    pub error: String,
    pub error_description: String,
}
