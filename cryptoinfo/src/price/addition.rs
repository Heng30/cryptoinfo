use crate::qobjmgr::{qobj, NodeType as QNodeType};
use platform_dirs::AppDirs;
use qmetaobject::*;
use super::data::{RawEthGas, RawBTCStats, RawMarket, RawLongShort, FearGreed, RawOtc};

#[allow(unused_imports)]
use ::log::{debug, warn};

#[derive(QObject, Default)]
pub struct Addition {
    base: qt_base_class!(trait QObject),

    greed_tody: qt_property!(i32; NOTIFY greed_tody_changed),
    greed_tody_changed: qt_signal!(),

    greed_yestoday: qt_property!(i32; NOTIFY greed_yestoday_changed),
    greed_yestoday_changed: qt_signal!(),

    total_market_cap_usd: qt_property!(i64; NOTIFY total_market_cap_usd_changed),
    total_market_cap_usd_changed: qt_signal!(),

    total_24h_volume_usd: qt_property!(i64; NOTIFY total_24h_volume_usd_changed),
    total_24h_volume_usd_changed: qt_signal!(),

    bitcoin_percentage_of_market_cap: qt_property!(f32; NOTIFY bitcoin_percentage_of_market_cap_changed),
    bitcoin_percentage_of_market_cap_changed: qt_signal!(),

    bitcoin_next_halving_days_left: qt_property!(i32; NOTIFY bitcoin_next_halving_days_left_changed),
    bitcoin_next_halving_days_left_changed: qt_signal!(),

    ahr999: qt_property!(f64; NOTIFY ahr999_changed),
    ahr999_changed: qt_signal!(),

    pub fear_greed_text: String,
    fear_greed_text_changed: qt_signal!(),

    pub market_text: String,
    market_text_changed: qt_signal!(),

    pub eth_gas_text: String,
    eth_gas_text_changed: qt_signal!(),

    pub btc_stats_text: String,
    btc_stats_text_changed: qt_signal!(),

    pub ahr999_text: String,
    ahr999_text_changed: qt_signal!(),

    pub long_short_text: String,
    long_short_text_changed: qt_signal!(),

    pub otc_text: String,
    otc_text_changed: qt_signal!(),

    low: qt_property!(u32; NOTIFY eth_gas_changed),
    average: qt_property!(u32; NOTIFY eth_gas_changed),
    fast: qt_property!(u32; NOTIFY eth_gas_changed),
    low_wait: qt_property!(u32; NOTIFY eth_gas_changed),
    average_wait: qt_property!(u32; NOTIFY eth_gas_changed),
    fast_wait: qt_property!(u32; NOTIFY eth_gas_changed),
    eth_gas_changed: qt_signal!(),

    long_rate: qt_property!(f32; NOTIFY long_short_changed),
    short_rate: qt_property!(f32; NOTIFY long_short_changed),
    long_vol_usd: qt_property!(f64; NOTIFY long_short_changed),
    short_vol_usd: qt_property!(f64; NOTIFY long_short_changed),
    long_short_symbol: qt_property!(QString; NOTIFY long_short_changed),
    long_short_changed: qt_signal!(),

    otc_usd: qt_property!(f32; NOTIFY otc_changed),
    otc_usdt: qt_property!(f32; NOTIFY otc_changed),
    otc_datetime: qt_property!(QString; NOTIFY otc_changed),
    otc_changed: qt_signal!(),

    update_fear_greed_qml: qt_method!(fn(&mut self)),
    update_market_qml: qt_method!(fn(&mut self)),
    update_eth_gas_qml: qt_method!(fn(&mut self)),
    update_btc_stats_qml: qt_method!(fn(&mut self)),
    update_ahr999_qml: qt_method!(fn(&mut self)),
    update_long_short_qml: qt_method!(fn(&mut self)),
    update_otc_qml: qt_method!(fn(&mut self)),
}

impl Addition {
    pub fn init_from_engine(engine: &mut QmlEngine, addtion: QObjectPinned<Addition>) {
        engine.set_object_property("price_addition".into(), addtion);
    }

    pub fn set_fear_greed_text(&mut self, text: String) {
        self.save2disk("fear-greed.json", &text);
        self.fear_greed_text = text;
        self.fear_greed_text_changed();
    }

    pub fn set_market_text(&mut self, text: String) {
        self.save2disk("market.json", &text);
        self.market_text = text;
        self.market_text_changed();
    }

    pub fn set_eth_gas_text(&mut self, text: String) {
        self.eth_gas_text = text;
        self.eth_gas_text_changed();
    }

    pub fn set_btc_stats_text(&mut self, text: String) {
        self.btc_stats_text = text;
        self.btc_stats_text_changed();
    }

    pub fn set_ahr999_text(&mut self, text: String) {
        self.ahr999_text = text;
        self.ahr999_text_changed();
    }

    pub fn set_long_short_text(&mut self, text: String) {
        self.long_short_text = text;
        self.long_short_text_changed();
    }

    pub fn set_otc_text(&mut self, text: String) {
        self.otc_text = text;
        self.otc_text_changed();
    }

    fn update_fear_greed_qml(&mut self) {
        if let Ok(fear_greed) = serde_json::from_str::<FearGreed>(&self.fear_greed_text) {
            let mut i = 0;
            for item in &fear_greed.data {
                if i == 0 {
                    self.greed_tody = item.value.parse().unwrap_or(0);
                    self.greed_tody_changed();
                }

                if i == 1 {
                    self.greed_yestoday = item.value.parse().unwrap_or(0);
                    self.greed_yestoday_changed();
                }
                i += 1;
            }
        }
    }

    fn update_market_qml(&mut self) {
        if let Ok(raw_market) = serde_json::from_str::<RawMarket>(&self.market_text) {
            self.total_market_cap_usd = raw_market.total_market_cap_usd;
            self.total_market_cap_usd_changed();

            self.total_24h_volume_usd = raw_market.total_24h_volume_usd;
            self.total_24h_volume_usd_changed();

            self.bitcoin_percentage_of_market_cap = raw_market.bitcoin_percentage_of_market_cap;
            self.bitcoin_percentage_of_market_cap_changed();
        }
    }

    fn update_eth_gas_qml(&mut self) {
        if let Ok(raw_eth_gas) = serde_json::from_str::<RawEthGas>(&self.eth_gas_text) {
            self.low = raw_eth_gas.low / 10;
            self.average = raw_eth_gas.average / 10;
            self.fast = raw_eth_gas.fast / 10;
            self.low_wait = (raw_eth_gas.low_wait * 60_f32) as u32;
            self.average_wait = (raw_eth_gas.average_wait * 60_f32) as u32;
            self.fast_wait = (raw_eth_gas.fast_wait * 60_f32) as u32;
            self.eth_gas_changed();
        }
    }

    fn update_btc_stats_qml(&mut self) {
        if let Ok(raw_btc_stats) = serde_json::from_str::<RawBTCStats>(&self.btc_stats_text) {
            let next_halving_blocks = if raw_btc_stats.n_blocks_total > 840_000 {
                1050_000_i32
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
            self.save2disk(
                "btc-next-halving-day-left.json",
                &("{".to_string()
                    + &format!("\"days\": {:?}", self.bitcoin_next_halving_days_left)
                    + "}"),
            );
        }
    }

    fn update_ahr999_qml(&mut self) {
        if let Ok(items) = serde_json::from_str::<Vec<Vec<f64>>>(&self.ahr999_text) {
            if let Some(item) = items.last() {
                if let Some(value) = item.last() {
                    self.ahr999 = *value;
                    self.ahr999_changed();
                }
            }
        }
    }

    fn update_long_short_qml(&mut self) {
        if let Ok(item) = serde_json::from_str::<RawLongShort>(&self.long_short_text) {
            if !item.success || item.data.is_empty() {
                return;
            }

            if let Some(item) = item.data.first() {
                self.long_short_symbol = item.symbol.clone().into();
                self.long_rate = item.long_rate;
                self.short_rate = item.short_rate;
                self.long_vol_usd = item.long_vol_usd;
                self.short_vol_usd = item.short_vol_usd;
                self.long_short_changed();
            }
        }

    }

    fn update_otc_qml(&mut self) {
        if let Ok(item) = serde_json::from_str::<RawOtc>(&self.otc_text) {
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

    fn save2disk(&self, file: &str, text: &str) {
        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        let path = app_dirs.data_dir.join(file).to_str().unwrap().to_string();
        if let Err(_) = std::fs::write(&path, &text) {
            warn!("save file {:?} failed", &path);
        };
    }
}
