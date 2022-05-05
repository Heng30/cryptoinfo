use reqwest;
use std::time::Duration;
use tokio::{self, time};

#[allow(unused_imports)]
use log::{debug, error, warn};

use crate::price::PriceAddition;
use crate::price::PriceModel;
use modeldata::QBox;

#[derive(Default, Debug, Copy, Clone)]
pub struct Download;

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

impl Download {
    pub fn update_price(&self, model: QBox<PriceModel>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let mut cnt = 0_u32;

            loop {
                let url = model.get().url.clone();
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

    pub fn update_fear_greed(&self, addition: QBox<PriceAddition>) {
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

    pub fn update_market(&self, addition: QBox<PriceAddition>) {
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

    pub fn update_eth_gas(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(30));
            let url = "https://ethgasstation.info/api/ethgasAPI.json?";

            loop {
                if let Ok(res) = http_get(&url).await {
                    addition.get_mut().set_eth_gas_text(res);
                }
                interval.tick().await;
            }
        });
    }

    pub fn update_btc_stats(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(60));
            let url = "https://blockchain.info/stats?format=json";

            loop {
                if let Ok(res) = http_get(&url).await {
                    addition.get_mut().set_btc_stats_text(res);
                }
                interval.tick().await;
            }
        });
    }
}
