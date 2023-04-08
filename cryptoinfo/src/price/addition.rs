use super::data::{
    FearGreed, RawBTCStats, RawBtcInfo, RawEthBurned, RawMarket, RawOtc, RawTotalBlast,
};

use crate::httpclient;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use ::log::warn;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;

#[derive(QObject, Default)]
pub struct Addition {
    base: qt_base_class!(trait QObject),

    greed_tody: qt_property!(i32; NOTIFY greed_changed),
    greed_yestoday: qt_property!(i32; NOTIFY greed_changed),
    greed_changed: qt_signal!(),

    total_market_cap_usd: qt_property!(i64; NOTIFY market_changed),
    total_24h_volume_usd: qt_property!(i64; NOTIFY market_changed),
    bitcoin_percentage_of_market_cap: qt_property!(f32; NOTIFY market_changed),
    market_changed: qt_signal!(),

    bitcoin_next_halving_days_left: qt_property!(i32; NOTIFY bitcoin_next_halving_days_left_changed),
    bitcoin_next_halving_days_left_changed: qt_signal!(),

    // 爆仓数据
    long_rate: qt_property!(f32; NOTIFY long_short_changed),
    short_rate: qt_property!(f32; NOTIFY long_short_changed),
    long_vol_usd: qt_property!(f64; NOTIFY long_short_changed),
    short_vol_usd: qt_property!(f64; NOTIFY long_short_changed),
    long_short_symbol: qt_property!(QString; NOTIFY long_short_changed),
    long_short_changed: qt_signal!(),

    // 场外usdt数据
    otc_usd: qt_property!(f32; NOTIFY otc_changed),
    otc_usdt: qt_property!(f32; NOTIFY otc_changed),
    otc_datetime: qt_property!(QString; NOTIFY otc_changed),
    otc_changed: qt_signal!(),

    // eth 燃烧数据
    eth_burned_total: qt_property!(f64; NOTIFY eth_burned_changed),
    eth_burned_rate_1h: qt_property!(f64; NOTIFY eth_burned_changed),
    eth_burned_rate_24h: qt_property!(f64; NOTIFY eth_burned_changed),
    eth_burned_changed: qt_signal!(),

    // btc hash数据
    btc_hash: qt_property!(QString; NOTIFY btc_info_changed),
    btc_hash_percent_24h: qt_property!(f64; NOTIFY btc_info_changed),
    btc_info_changed: qt_signal!(),

    // btc逃顶指数
    btc_ma730: qt_property!(f64; NOTIFY btc_ma730_changed),
    btc_ma730_mu5: qt_property!(f64; NOTIFY btc_ma730_changed),
    btc_ma730_price: qt_property!(f64; NOTIFY btc_ma730_changed),
    btc_ma730_create_time: qt_property!(u64; NOTIFY btc_ma730_changed),
    btc_ma730_changed: qt_signal!(),

    // btc 挖矿成本
    btc_mining_cost: qt_property!(f64; NOTIFY btc_mining_cost_changed),
    btc_mining_cost_changed: qt_signal!(),

    // 爆仓数据
    total_blast_1h: qt_property!(f64; NOTIFY total_blast_changed),
    total_blast_24h: qt_property!(f64; NOTIFY total_blast_changed),
    total_blast_num_24h: qt_property!(u32; NOTIFY total_blast_changed),
    total_blast_update_time: qt_property!(u64; NOTIFY total_blast_changed),
    total_blast_changed: qt_signal!(),
}

impl Addition {
    pub fn init_from_engine(engine: &mut QmlEngine, addtion: QObjectPinned<Addition>) {
        engine.set_object_property("price_addition".into(), addtion);
    }

    pub fn init(&mut self) {
        self.async_fear_greed();
        self.async_market();
        self.async_eth_burned();
        self.asyn_btc_stats();
        self.async_btc_info();
        self.async_otc();
        self.async_total_blast();
    }

    fn async_fear_greed(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_fear_greed(text);
        });

        let url = "https://api.alternative.me/fng/?limit=2".to_string();
        httpclient::download_timer(url, 30, 5, cb);
    }

    pub fn async_market(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_market(text);
        });

        let url = "https://api.alternative.me/v1/global/".to_string();
        httpclient::download_timer(url, 30, 5, cb);
    }

    pub fn async_eth_burned(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_eth_burned(text);
        });

        let url = "https://api.btc126.vip/etherchain.php?from=ethburn".to_string();
        httpclient::download_timer(url, 60, 5, cb);
    }

    pub fn asyn_btc_stats(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_btc_stats(text);
        });

        let url = "https://blockchain.info/stats?format=json".to_string();
        httpclient::download_timer(url, 3600, 5, cb);
    }

    fn async_btc_info(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_btc_info(text);
        });

        let url = "https://api.btc126.vip/oklink.php?from=poolinfo".to_string();
        httpclient::download_timer(url, 600, 5, cb);
    }

    fn async_otc(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_otc(text);
        });

        let url = "https://history.btc123.fans/usdt/api.php".to_string();
        httpclient::download_timer(url, 600, 5, cb);
    }

    fn async_total_blast(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_total_blast(text);
        });

        let url = "https://api.btc126.vip/bicoin.php?from=24hbaocang".to_string();
        httpclient::download_timer(url, 1800, 5, cb);
    }

    fn update_fear_greed(&mut self, text: String) {
        if let Ok(fear_greed) = serde_json::from_str::<FearGreed>(&text) {
            for (i, item) in fear_greed.data.iter().enumerate() {
                if i == 0 {
                    self.greed_tody = item.value.parse().unwrap_or(0);
                }

                if i == 1 {
                    self.greed_yestoday = item.value.parse().unwrap_or(0);
                }
                self.greed_changed();

                if i == 1 {
                    break;
                }
            }
        }
    }

    fn update_market(&mut self, text: String) {
        if let Ok(raw_market) = serde_json::from_str::<RawMarket>(&text) {
            self.total_market_cap_usd = raw_market.total_market_cap_usd;
            self.total_24h_volume_usd = raw_market.total_24h_volume_usd;

            self.bitcoin_percentage_of_market_cap = raw_market.bitcoin_percentage_of_market_cap;
            self.market_changed();
        }
    }

    fn update_eth_burned(&mut self, text: String) {
        let wei_per_eth = 1e18_f64;
        if let Ok(item) = serde_json::from_str::<RawEthBurned>(&text) {
            self.eth_burned_total = item.total_burned / wei_per_eth;
            self.eth_burned_rate_1h = item.burn_rate_1_h / wei_per_eth;
            self.eth_burned_rate_24h = item.burn_rate_24_h / wei_per_eth;
            self.eth_burned_changed();
        }
    }

    fn update_btc_stats(&mut self, text: String) {
        if let Ok(raw_btc_stats) = serde_json::from_str::<RawBTCStats>(&text) {
            let next_halving_blocks = if raw_btc_stats.n_blocks_total > 840_000 {
                1_050_000_i32
            } else {
                840_000_i32
            };
            let blocks_left = next_halving_blocks - raw_btc_stats.n_blocks_total as i32;
            if blocks_left < 0 {
                self.bitcoin_next_halving_days_left = -1;
            } else {
                self.bitcoin_next_halving_days_left = (blocks_left as f32
                    * raw_btc_stats.minutes_between_blocks
                    / (60.0 * 24.0)) as i32;
            }
            self.bitcoin_next_halving_days_left_changed();
        }
    }

    fn update_otc(&mut self, text: String) {
        if let Ok(item) = serde_json::from_str::<RawOtc>(&text) {
            if item.data.is_empty() {
                return;
            }

            if let Some(item) = item.data.last() {
                self.otc_usd = item.usd.to_string().parse::<f32>().unwrap_or(0.0_f32);
                self.otc_usdt = item.usdt.to_string().parse::<f32>().unwrap_or(0.0_f32);
                self.otc_datetime = item.datetime.clone().into();
                self.otc_changed();
            }
        }
    }

    fn update_btc_info(&mut self, text: String) {
        if let Ok(item) = serde_json::from_str::<RawBtcInfo>(&text) {
            self.btc_hash_percent_24h = item.data.hashes.global_hashes_percent_change_24h;
            self.btc_hash = item.data.hashes.global_hashes.into();
            self.btc_info_changed();
        }
    }

    fn update_total_blast(&mut self, text: String) {
        if let Ok(item) = serde_json::from_str::<RawTotalBlast>(&text) {
            self.total_blast_1h = item.data.total_blast_1h;
            self.total_blast_24h = item.data.total_blast_24h;
            self.total_blast_num_24h = item.data.total_blast_num_24h;
            self.total_blast_update_time = item.data.update_time / 1000;
            self.total_blast_changed();
        }
    }

    #[allow(unused)]
    fn save2disk(&self, file: &str, text: &str) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        let path = app_dirs.data_dir.join(file).to_str().unwrap().to_string();
        if let Err(e) = std::fs::write(&path, &text) {
            warn!("save file {:?} failed, error: {:?}", &path, e);
        };
    }
}
