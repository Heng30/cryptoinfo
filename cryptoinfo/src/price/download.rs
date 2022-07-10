use crate::price::PriceAddition;
use crate::price::PriceModel;
#[allow(unused_imports)]
use log::{debug, error, warn};
use modeldata::QBox;
use reqwest;
use std::time::Duration;
use tokio::{self, time};

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
                let url = model.borrow().url.clone();
                if model.borrow().update_now {
                    if let Ok(res) = http_get(&url).await {
                        model.borrow_mut().update_text(res);
                    }
                    model.borrow_mut().update_now = false;
                    continue;
                }

                if model.borrow().update_interval != 0 && cnt % model.borrow().update_interval == 3
                {
                    if let Ok(res) = http_get(&url).await {
                        model.borrow_mut().update_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_fear_greed(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let url = "https://api.alternative.me/fng/?limit=2";
            let mut cnt = 0;

            loop {
                if cnt % 30 == 5 {
                    if let Ok(res) = http_get(&url).await {
                        addition.borrow_mut().set_fear_greed_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_market(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let url = "https://api.alternative.me/v1/global/";
            let mut cnt = 0;

            loop {
                if cnt % 30 == 5 {
                    if let Ok(res) = http_get(&url).await {
                        addition.borrow_mut().set_market_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_eth_gas(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let url = "https://ethgasstation.info/api/ethgasAPI.json?";
            let mut cnt = 0;

            loop {
                if cnt % 30 == 5 {
                    if let Ok(res) = http_get(&url).await {
                        addition.borrow_mut().set_eth_gas_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_eth_burned(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let url = "https://api.btc126.vip/etherchain.php?from=ethburn";
            let mut cnt = 0;

            loop {
                if cnt % 60 == 5 {
                    if let Ok(res) = http_get(&url).await {
                        addition.borrow_mut().set_eth_burned_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_btc_stats(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let url = "https://blockchain.info/stats?format=json";
            let mut cnt = 0;

            loop {
                if cnt % 60 == 5 {
                    if let Ok(res) = http_get(&url).await {
                        addition.borrow_mut().set_btc_stats_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_ahr999(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let url = "http://ahr999mixin.tk/data.json";
            let mut cnt = 0;

            loop {
                if cnt % 3600 == 5 {
                    if let Ok(res) = http_get(&url).await {
                        addition.borrow_mut().set_ahr999_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_long_short(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let url = "https://api.btc126.vip/bybt.php?from=24h";
            let mut cnt = 0;

            loop {
                if cnt % 600 == 5 {
                    if let Ok(res) = http_get(&url).await {
                        addition.borrow_mut().set_long_short_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_otc(&self, addition: QBox<PriceAddition>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));
            let url = "https://history.btc123.fans/usdt/api.php";
            let mut cnt = 0;

            loop {
                if cnt % 600 == 5 {
                    if let Ok(res) = http_get(&url).await {
                        addition.borrow_mut().set_otc_text(res);
                    } else {
                        cnt = 0;
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }
}
