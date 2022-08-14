use crate::account::okex_rest_header_sign;
use crate::config::Config;
use crate::qobjmgr::{qobj, NodeType};
use ::log::debug;
use chrono::prelude::{DateTime, Local, Utc};
use reqwest;
use reqwest::header::HeaderMap;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use tokio::{self, time};

pub trait DownloadProvider {
    fn url(&self) -> String;
    fn update_interval(&self) -> usize;
    fn update_now(&self) -> bool;
    fn disable_update_now(&self);
    fn parse_body(&mut self, _text: &str) {}
}

pub trait PostContentProvider {
    fn content(&mut self) -> String;
}

pub trait HeaderProvider {
    fn headers(&mut self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36".parse().unwrap());
        h
    }
}

pub trait OkexDownloadProvider {
    fn path(&self) -> String;
}

pub async fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert("User-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.61 Safari/537.36".parse().unwrap());
    let res = client
        .get(url)
        .headers(headers)
        .timeout(Duration::new(15, 0))
        .send()
        .await?
        .text()
        .await?;
    return Ok(res);
}

pub async fn http_post(
    url: &str,
    headers: HeaderMap,
    content: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .headers(headers)
        .body(content)
        .timeout(Duration::new(15, 0))
        .send()
        .await?
        .text()
        .await?;
    return Ok(res);
}

pub fn download_timer(
    url: String,
    interval: usize,
    delay_start_second: usize,
    cb: impl Fn(String) + Send + Sync + Clone + 'static,
) {
    tokio::spawn(async move {
        let mut second = time::interval(time::Duration::from_secs(1));
        let mut cnt = 0usize;
        let interval = usize::max(1, interval);

        loop {
            if cnt % interval == delay_start_second {
                if let Ok(res) = http_get(&url).await {
                    if !res.is_empty() {
                        cb(res);
                    }
                } else {
                    cnt = 0;
                }
            }
            cnt += 1;
            second.tick().await;
        }
    });
}

pub fn download_timer_pro(
    mut provider: impl DownloadProvider + Send + Clone + 'static,
    delay_start_second: usize,
    cb: impl Fn(String) + Send + Sync + Clone + 'static,
) {
    tokio::spawn(async move {
        let mut second = time::interval(time::Duration::from_secs(1));
        let mut cnt = 0usize;
        loop {
            let url = &provider.url();
            let interval = usize::max(1, provider.update_interval());
            if provider.update_now() {
                if let Ok(res) = http_get(url).await {
                    if !res.is_empty() {
                        provider.parse_body(&res);
                        cb(res);
                        cnt = delay_start_second + 1;
                    }
                }
                provider.disable_update_now();
                continue;
            }

            if cnt % interval == delay_start_second {
                if let Ok(res) = http_get(url).await {
                    if !res.is_empty() {
                        provider.parse_body(&res);
                        cb(res);
                    }
                } else {
                    cnt = 0;
                }
            }
            cnt += 1;
            second.tick().await;
        }
    });
}

fn add_okex_header_field(headers: &mut HeaderMap, meth_url: &str) -> bool {
    let conf = qobj::<Config>(NodeType::Config);
    if conf.okex_api_key.is_empty()
        || conf.okex_passphrase.is_empty()
        || conf.okex_secret_key.is_empty()
    {
        debug!("okex api info is invalid!");
        return false;
    }

    let dt: DateTime<Utc> = Local::now().into();
    let timestamp = format!("{}", dt.format("%+"))
        .rsplit('.')
        .collect::<Vec<&str>>()
        .last()
        .unwrap_or(&"")
        .to_string()
        + ".000Z";
    let sign = okex_rest_header_sign(&timestamp, meth_url, &conf.okex_secret_key.to_string());

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
    return true;
}

async fn http_get_okex(url: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("User-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.61 Safari/537.36".parse().unwrap());

    if !add_okex_header_field(&mut headers, &format!("GET{}", path)) {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "okex api info invalid!",
        )));
    }

    let res = client
        .get(url)
        .headers(headers)
        .timeout(Duration::new(15, 0))
        .send()
        .await?
        .text()
        .await?;
    return Ok(res);
}

pub fn download_timer_okex_pro(
    mut provider: impl DownloadProvider + OkexDownloadProvider + Send + Clone + 'static,
    delay_start_second: usize,
    cb: impl Fn(String) + Send + Sync + Clone + 'static,
) {
    tokio::spawn(async move {
        let mut second = time::interval(time::Duration::from_secs(1));
        let mut cnt = 0usize;
        loop {
            let url = &provider.url();
            let path = &provider.path();
            let interval = usize::max(1, provider.update_interval());
            if provider.update_now() {
                match http_get_okex(url, path).await {
                    Ok(res) => {
                        if !res.is_empty() {
                            provider.parse_body(&res);
                            cb(res);
                            cnt = delay_start_second + 1;
                        }
                    }
                    Err(e) => debug!("{:?}", e),
                }
                provider.disable_update_now();
                continue;
            }

            if cnt % interval == delay_start_second {
                match http_get_okex(url, path).await {
                    Ok(res) => {
                        if !res.is_empty() {
                            provider.parse_body(&res);
                            cb(res);
                        }
                    }
                    Err(e) => {
                        cnt = 0;
                        debug!("{:?}", e);
                    }
                }
            }
            cnt += 1;
            second.tick().await;
        }
    });
}

pub fn post(
    mut provider: impl DownloadProvider + PostContentProvider + HeaderProvider + Send + Clone + 'static,
    delay_start_second: usize,
    cb: impl Fn(String) + Send + Sync + Clone + 'static,
) {
    tokio::spawn(async move {
        let mut second = time::interval(time::Duration::from_secs(1));
        let mut cnt = 0usize;
        loop {
            let url = &provider.url();
            let headers = provider.headers();
            let content = provider.content();
            let interval = usize::max(1, provider.update_interval());
            if provider.update_now() {
                match http_post(url, headers, content).await {
                    Ok(res) => {
                        if !res.is_empty() {
                            provider.parse_body(&res);
                            cb(res);
                            cnt = delay_start_second + 1;
                        }
                    }
                    Err(e) => debug!("{:?}", e),
                }
                provider.disable_update_now();
                continue;
            }

            if cnt % interval == delay_start_second {
                match http_post(url, headers, content).await {
                    Ok(res) => {
                        if !res.is_empty() {
                            provider.parse_body(&res);
                            cb(res);
                        }
                    }
                    Err(e) => {
                        cnt = 0;
                        debug!("{:?}", e);
                    }
                }
            }
            cnt += 1;
            second.tick().await;
        }
    });
}
