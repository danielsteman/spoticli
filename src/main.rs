use reqwest;
use base64;
use std::collections::HashMap;

async fn songs() {
    let result = reqwest::get("https://api.spotify.com/v1/search").await;
    println!("{:?}", result);
}

async fn auth() {
    let client_id = "test";
    let client_secret = "test";
    let encoded = base64::encode(format!("{}:{}", client_id, client_secret));

    let mut headers = HashMap::from([
        ("url", "https://accounts.spotify.com/api/token"),
        ("Authorization", &*format!("{}{}", "Basic ", encoded)),
    ]);
}

#[tokio::main]
async fn main() {
    auth().await
}