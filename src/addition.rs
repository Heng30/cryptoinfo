use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

#[allow(unused_imports)]
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct FearGreed {
    data: Vec<RawGreed>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawGreed {
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawMarket {
    total_market_cap_usd: i64,
    total_24h_volume_usd: i64,
    bitcoin_percentage_of_market_cap: f32,
}

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

    system_time: qt_property!(QString; NOTIFY system_time_changed),
    system_time_changed: qt_signal!(),

    pub fear_greed_text: String,
    fear_greed_text_changed: qt_signal!(),

    pub market_text: String,
    market_text_changed: qt_signal!(),

    update_fear_greed: qt_method!(
        fn update_fear_greed(&mut self) {
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
    ),

    update_market: qt_method!(
        fn update_market(&mut self) {
            if let Ok(raw_market) = serde_json::from_str::<RawMarket>(&self.market_text) {
                self.total_market_cap_usd = raw_market.total_market_cap_usd;
                self.total_market_cap_usd_changed();

                self.total_24h_volume_usd = raw_market.total_24h_volume_usd;
                self.total_24h_volume_usd_changed();

                self.bitcoin_percentage_of_market_cap = raw_market.bitcoin_percentage_of_market_cap;
                self.bitcoin_percentage_of_market_cap_changed();
            }
        }
    ),

    update_time: qt_method!(
        fn update_time(&mut self) {
            self.system_time =
                format!("{}", Local::now().format("%m-%d %H:%M:%S").to_string()).into();
            self.system_time_changed();
        }
    ),

    get_time_from_utc_seconds: qt_method!(
        fn get_time_from_utc_seconds(&self, sec: i64) -> QString {
            let time = FixedOffset::east(8 * 3600).timestamp(sec, 0);
            return format!("{}", time.format("%m-%d %H:%M").to_string()).into();
        }
    ),
}

impl Addition {
    pub fn init_from_engine(engine: &mut QmlEngine, addtion: QObjectPinned<Addition>) {
        engine.set_object_property("pricer_addition".into(), addtion);
    }

    pub fn set_fear_greed_text(&mut self, text: String) {
        self.fear_greed_text = text;
        self.fear_greed_text_changed();
    }

    pub fn set_market_text(&mut self, text: String) {
        self.market_text = text;
        self.market_text_changed();
    }
}
