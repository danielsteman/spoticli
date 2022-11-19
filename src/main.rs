use base64;
use dotenv::dotenv;
use reqwest;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;

async fn songs() {
    let result = reqwest::get("https://api.spotify.com/v1/search").await;
    println!("{:?}", result);
}

#[derive(Serialize, Deserialize, Debug)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

async fn get_token() -> String {
    let client_id = std::env::var("CLIENT_ID").expect("Client ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("Client secret must be set");

    let encoded = base64::encode(format!("{}:{}", client_id, client_secret));
    let client = Client::new();
    let params = [("grant_type", "client_credentials")];

    let res = client.post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("{}{}", "Basic ", encoded))
        .form(&params)
        .send()
        .await
        .expect("Failed to get token")
        .text()
        .await
        .expect("Failed to get payload");

    let parsed_res: Token = serde_json::from_str(&res).expect("Failed to parse token in payload");
    parsed_res.access_token
}

// let scope = "user-read-private user-read-email";
// let redirect_uri = "http://localhost:8888/callback";

// let query = vec![
//     "response_type", "code",
//     "client_id", &client_id,
//     "scope", &scope,
//     "redirect_uri", &redirect_uri,
// ];

// let response = client.get("https://accounts.spotify.com/authorize?")
//     .query(&query)
//     .send()
//     .await;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = get_token().await;
    println!("{:?}", token);
}
