use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CaptchaBalance {
    pub errorId: i32,
    pub errorCode: Option<String>,
    pub balance: f32,
}

pub fn get_balance(client_key: &str) -> Result<CaptchaBalance, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client.post("https://api.capmonster.cloud/getBalance")
        .json(&serde_json::json!({ "clientKey": client_key }))
        .send()?
        .json::<CaptchaBalance>()
}