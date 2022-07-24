#[allow(unused_imports)]
use ::log::{debug, warn};
use base64;
use chrono::Local;
use hmac_sha256::HMAC;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OkexLoginReqApiMsg {
    #[serde(rename(serialize = "apiKey", deserialize = "apiKey"))]
    pub api_key: String,
    pub passphrase: String,
    pub timestamp: String,
    pub sign: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OkexLoginReqMsg {
    pub op: String,
    pub args: Vec<OkexLoginReqApiMsg>,
}

impl OkexLoginReqMsg {
    pub fn new(passphrase: &str, api_key: &str, secret_key: &str) -> String {
        let timestamp = format!("{}", Local::now().timestamp());
        let sign = base64::encode(HMAC::mac(
            format!("{}GET/users/self/verify", &timestamp),
            secret_key,
        ));
        let msg = Self {
            op: "login".to_string(),
            args: vec![OkexLoginReqApiMsg {
                api_key: api_key.to_string(),
                passphrase: passphrase.to_string(),
                timestamp: timestamp,
                sign: sign,
            }],
        };

        match serde_json::to_string(&msg) {
            Ok(jstr) => return jstr,
            Err(e) => debug!("{:?}", e),
        };
        return String::default();
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OkexResMsgEvent {
    pub event: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OkexLoginResMsg {
    pub event: String,
    pub code: String,
    pub msg: String,
}
