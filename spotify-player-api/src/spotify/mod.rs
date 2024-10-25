mod auth;
mod token;

use axum::{
    routing::{get, post},
    Router,
};

pub fn new_handler() -> Router {
    Router::new()
        .route("/auth", get(auth::handler::authorize))
        .route("/token", post(token::handler::get_access_token))
}
