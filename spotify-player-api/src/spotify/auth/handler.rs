use axum::response::Redirect;
use rand::Rng;
use std::env;

pub async fn authorize() -> Redirect {
    let client = reqwest::Client::new();
    let resp = client
        .get(env::var("SPOTIFY_URL_AUTH").unwrap())
        .query(&[
            ("response_type", "code"),
            ("client_id", env::var("SPOTIFY_CLIENT_ID").unwrap().as_str()),
            ("scope", env::var("SPOTIFY_SCOPE").unwrap().as_str()),
            (
                "redirect_uri",
                env::var("SPOTIFY_REDIRECT_URI").unwrap().as_str(),
            ),
            ("state", generate_random_string(16).as_str()),
        ])
        .send()
        .await
        .unwrap();

    Redirect::temporary(resp.url().as_str())
}

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
