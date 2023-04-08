use super::okex_rest_header_sign;
use crate::config::Config;
use crate::httpclient;
use crate::qobjmgr::{qobj, NodeType};
use chrono::prelude::{DateTime, Local, Utc};
use reqwest::header::HeaderMap;

pub fn get_headers(path: &str) -> HeaderMap {
    let conf = qobj::<Config>(NodeType::Config);
    if conf.okex_api_key.is_empty()
        || conf.okex_passphrase.is_empty()
        || conf.okex_secret_key.is_empty()
    {
        return httpclient::common_headers();
    }

    let dt: DateTime<Utc> = Local::now().into();
    let timestamp = format!("{}", dt.format("%+"))
        .rsplit('.')
        .collect::<Vec<&str>>()
        .last()
        .unwrap_or(&"")
        .to_string()
        + ".000Z";
    let sign = okex_rest_header_sign(
        &timestamp,
        &format!("GET{}", path),
        &conf.okex_secret_key.to_string(),
    );

    let mut headers = httpclient::common_headers();
    headers.insert(
        "OK-ACCESS-KEY",
        conf.okex_api_key.to_string().parse().unwrap(),
    );
    headers.insert("OK-ACCESS-SIGN", sign.parse().unwrap());
    headers.insert("OK-ACCESS-TIMESTAMP", timestamp.parse().unwrap());
    headers.insert(
        "OK-ACCESS-PASSPHRASE",
        conf.okex_passphrase.to_string().parse().unwrap(),
    );
    headers
}
