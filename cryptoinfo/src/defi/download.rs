use reqwest;
use std::time::Duration;
use tokio::{self, time};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::{DefiChainModel, DefiChainTVLModel, DefiProtocolModel};
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
    pub fn update_defi_protocol(&self, model: QBox<DefiProtocolModel>) {
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

                if model.borrow().update_interval != 0 && cnt % model.borrow().update_interval == 0
                {
                    if let Ok(res) = http_get(&url).await {
                        model.borrow_mut().update_text(res);
                    }
                }
                cnt += 1;
                interval.tick().await;
            }
        });
    }

    pub fn update_defi_chain(&self, model: QBox<DefiChainModel>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));

            loop {
                let url = model.borrow().url.clone();
                if model.borrow().update_now {
                    if let Ok(res) = http_get(&url).await {
                        model.borrow_mut().update_text(res);
                    }
                    model.borrow_mut().update_now = false;
                }

                interval.tick().await;
            }
        });
    }

    pub fn update_defi_chain_tvl(&self, model: QBox<DefiChainTVLModel>) {
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(1));

            loop {
                let url = model.borrow().gen_url();
                let name = model.borrow().name.to_string();

                if model.borrow().update_now && !url.is_empty() {
                    if let Ok(res) = http_get(&url).await {
                        model.borrow_mut().update_text(name, res);
                    }
                    model.borrow_mut().update_now = false;
                }

                interval.tick().await;
            }
        });
    }
}
