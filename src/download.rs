use reqwest;
use std::time::Duration;
use tokio::{self, time};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::addition::Addition as pricer_addition;
use crate::pricer::Model as pricer_model;
use crate::qbox::QBox;

async fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .timeout(Duration::new(15, 0))
        .send()
        .await?
        .text()
        .await?;
    return Ok(res);
}

pub fn update_price(model: QBox<pricer_model>) {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(1));
        let mut cnt = 0_u32;

        loop {
            let url = model.get().price_url.clone();
            if model.get().update_now {
                if let Ok(res) = http_get(&url).await {
                    model.get_mut().update_text(res);
                }
                model.get_mut().update_now = false;
                continue;
            }

            if model.get().update_interval != 0 && cnt % model.get().update_interval == 0 {
                if let Ok(res) = http_get(&url).await {
                    model.get_mut().update_text(res);
                }
            }
            cnt += 1;
            interval.tick().await;
        }
    });
}

pub fn update_fear_greed(addition: QBox<pricer_addition>) {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(30));
        let url = "https://api.alternative.me/fng/?limit=2";

        loop {
            if let Ok(res) = http_get(&url).await {
                addition.get_mut().set_fear_greed_text(res);
            }
            interval.tick().await;
        }
    });
}

pub fn update_market(addition: QBox<pricer_addition>) {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(30));
        let url = "https://api.alternative.me/v1/global/";

        loop {
            if let Ok(res) = http_get(&url).await {
                addition.get_mut().set_market_text(res);
            }
            interval.tick().await;
        }
    });
}
