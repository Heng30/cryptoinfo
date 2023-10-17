use super::data::{FearGreed, RawMarket, RawOtc};
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

    // 场外usdt数据
    otc_usd: qt_property!(f32; NOTIFY otc_changed),
    otc_usdt: qt_property!(f32; NOTIFY otc_changed),
    otc_datetime: qt_property!(QString; NOTIFY otc_changed),
    otc_changed: qt_signal!(),
}

impl Addition {
    pub fn init_from_engine(engine: &mut QmlEngine, addtion: QObjectPinned<Addition>) {
        engine.set_object_property("price_addition".into(), addtion);
    }

    pub fn init(&mut self) {
        self.async_fear_greed();
        self.async_market();
        self.async_otc();
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

    fn async_otc(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_otc(text);
        });

        let url = "https://history.btc123.fans/usdt/api.php".to_string();
        httpclient::download_timer(url, 600, 5, cb);
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

    #[allow(unused)]
    fn save2disk(&self, file: &str, text: &str) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        let path = app_dirs.data_dir.join(file).to_str().unwrap().to_string();
        if let Err(e) = std::fs::write(&path, &text) {
            warn!("save file {:?} failed, error: {:?}", &path, e);
        };
    }
}
