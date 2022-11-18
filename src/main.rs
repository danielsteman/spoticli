use base64;
use dotenv::dotenv;
use reqwest;
use reqwest::Client;
use std::collections::HashMap;

async fn songs() {
    let result = reqwest::get("https://api.spotify.com/v1/search").await;
    println!("{:?}", result);
}

async fn auth() {
    let client_id = std::env::var("CLIENT_ID").expect("Client ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("Client secret must be set");
    let encoded = base64::encode(format!("{}:{}", client_id, client_secret));

    let headers = HashMap::from([
        ("url", "https://accounts.spotify.com/api/token"),
        ("Authorization", &*format!("{}{}", "Basic ", encoded)),
    ]);

    let scope = "user-read-private user-read-email";
    let redirect_uri = "http://localhost:8888/callback";

    let query = vec![
        "response_type", "code",
        "client_id", &client_id,
        "scope", &scope,
        "redirect_uri", &redirect_uri,
    ];

    let client = Client::new();

    let response = client.get("https://accounts.spotify.com/authorize?")
        .query(&query)
        .send()
        .await;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    auth().await
}
