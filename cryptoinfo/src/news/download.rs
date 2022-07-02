use crate::news::NewsModel;
#[allow(unused_imports)]
use log::{debug, warn};
use modeldata::QBox;
use reqwest::header::HeaderMap;
use std::time::Duration;
use tokio::{self, time};

#[derive(Default, Debug, Copy, Clone)]
pub struct Download;

async fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.61 Safari/537.36".parse().unwrap());
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

impl Download {
    pub fn update_news(&self, model: QBox<NewsModel>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let mut cnt = 0_u32;

            loop {
                let url = model.borrow().url.clone();
                if model.borrow().update_now {
                    if let Ok(res) = http_get(&url).await {
                        let res = NewsModel::parse_hmtl_lvdong(&res);
                        model.borrow_mut().update_text(res);
                    }
                    model.borrow_mut().update_now = false;
                    continue;
                }

                if model.borrow().update_interval != 0 && cnt % model.borrow().update_interval == 0
                {
                    if let Ok(res) = http_get(&url).await {
                        let res = NewsModel::parse_hmtl_lvdong(&res);
                        model.borrow_mut().update_text(res);
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

}
