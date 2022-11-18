use base64;
use dotenv::dotenv;
use reqwest;
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


}

#[tokio::main]
async fn main() {
    dotenv().ok();
    auth().await
}
