use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    Json,
};
use base64::prelude::*;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::{collections::HashMap, env};

use super::model;

pub async fn get_access_token(
    Json(payload): Json<model::GetTokenRequest>,
) -> (StatusCode, Json<model::GetAccessTokenResponse>) {
    let basic = generate_auth_base64(
        env::var("SPOTIFY_CLIENT_ID").unwrap().as_str(),
        env::var("SPOTIFY_CLIENT_SECRET").unwrap().as_str(),
    );

    let auth = HeaderValue::from_str(&basic).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(AUTHORIZATION, auth);

    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI").unwrap();

    let mut params = HashMap::new();
    params.insert("code", payload.code.as_str());
    params.insert("redirect_uri", redirect_uri.as_str());
    params.insert("grant_type", "authorization_code");

    let client = reqwest::Client::new();
    let resp = client
        .post(env::var("SPOTIFY_URL_TOKEN").unwrap().as_str())
        .headers(headers)
        .form(&params)
        .send()
        .await;

    match resp {
        Ok(response) if response.status().is_success() => {
            let data = response
                .json::<model::GetAccessTokenResponseSuccess>()
                .await;
            (
                StatusCode::OK,
                Json(model::GetAccessTokenResponse::Success(data.unwrap())),
            )
        }
        Ok(response) => {
            let data = response.json::<model::GetAccessTokenResponseError>().await;

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(model::GetAccessTokenResponse::Error(data.unwrap())),
            )
        }
        Err(e) => {
            let data = model::GetAccessTokenResponseError {
                error: String::from("unexpected_error"),
                error_description: e.to_string(),
            };

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(model::GetAccessTokenResponse::Error(data)),
            )
        }
    }
}

fn generate_auth_base64(client_id: &str, client_secret: &str) -> String {
    format!(
        "Basic {}",
        BASE64_STANDARD.encode(format!("{}:{}", client_id, client_secret))
    )
}
