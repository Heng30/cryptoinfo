use reqwest;
use std::time::Duration;
use tokio::{self, time};
use reqwest::header::HeaderMap;

pub trait DownloadProvider {
    fn url(&self) -> String;
    fn update_interval(&self) -> usize;
    fn update_now(&self) -> bool;
    fn disable_update_now(&self);
    fn parse_body(&mut self, _text: &str) {}
}

pub async fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "accept",
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.61 Safari/537.36".parse().unwrap());
    let res = client
        .get(url)
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
