use ::log::debug;
use reqwest;
use reqwest::header::HeaderMap;
use std::time::Duration;
use tokio::{self, time};

pub trait DownloadProvider {
    fn url(&self) -> String;
    fn update_interval(&self) -> usize;
    fn update_now(&self) -> bool;
    fn disable_update_now(&self);
    fn parse_body(&mut self, _text: &str) {}

    fn headers(&mut self) -> HeaderMap {
        common_headers()
    }
}

pub trait PostContentProvider {
    fn content(&mut self) -> String;
}

pub fn common_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36".parse().unwrap());
    headers
}

pub async fn http_get(url: &str, headers: HeaderMap) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::Client::new()
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
        let headers = common_headers();

        loop {
            if cnt % interval == delay_start_second {
                match http_get(&url, headers.clone()).await {
                    Ok(res) => {
                        if !res.is_empty() {
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

pub fn download_timer_pro(
    mut provider: impl DownloadProvider + Send + Clone + 'static,
    delay_start_second: usize,
    cb: impl Fn(String) + Send + Sync + Clone + 'static,
) {
    tokio::spawn(async move {
        let mut second = time::interval(time::Duration::from_secs(1));
        let mut cnt = 0usize;
        let mut try_cnt = 0;
        let max_try_cnt = 5;
        loop {
            let url = &provider.url();
            let interval = usize::max(1, provider.update_interval());
            let headers = provider.headers();
            if provider.update_now() {
                match http_get(url, headers).await {
                    Ok(res) => {
                        if !res.is_empty() {
                            try_cnt = 0;
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

            if cnt % interval == delay_start_second && try_cnt < max_try_cnt {
                match http_get(url, headers).await {
                    Ok(res) => {
                        if !res.is_empty() {
                            try_cnt = 0;
                            provider.parse_body(&res);
                            cb(res);
                        }
                    }
                    Err(e) => {
                        cnt = 0;
                        try_cnt += 1;
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
    mut provider: impl DownloadProvider + PostContentProvider + Send + Clone + 'static,
    delay_start_second: usize,
    cb: impl Fn(String) + Send + Sync + Clone + 'static,
) {
    tokio::spawn(async move {
        let mut second = time::interval(time::Duration::from_secs(1));
        let mut cnt = 0usize;
        let mut try_cnt = 0;
        let max_try_cnt = 5;
        loop {
            let url = &provider.url();
            let headers = provider.headers();
            let content = provider.content();
            let interval = usize::max(1, provider.update_interval());
            if provider.update_now() {
                match http_post(url, headers, content).await {
                    Ok(res) => {
                        if !res.is_empty() {
                            try_cnt = 0;
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

            if cnt % interval == delay_start_second && try_cnt < max_try_cnt {
                match http_post(url, headers, content).await {
                    Ok(res) => {
                        if !res.is_empty() {
                            try_cnt = 0;
                            provider.parse_body(&res);
                            cb(res);
                        }
                    }
                    Err(e) => {
                        cnt = 0;
                        try_cnt += 1;
                        debug!("{:?}", e);
                    }
                }
            }
            cnt += 1;
            second.tick().await;
        }
    });
}
