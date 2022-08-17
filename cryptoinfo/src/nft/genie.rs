use super::data::{NFTGenieRawItem as RawItem, NTFGenieItem as Item};
use crate::httpclient;
use crate::utility::Utility;
use ::log::debug;
use modeldata::*;
use qmetaobject::*;
use reqwest::header::HeaderMap;

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        url: String,
    }, members_qt: {
        bull_percent: [f32; bull_percent_changed],
        update_now: [bool; update_now_changed], // 马上更新
        update_time: [QString; update_time_changed], // 数据更新时间
    }, signals_qt: {
    }, methods_qt: {
        refresh_qml: fn(&mut self),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow().url.clone();
    }

    fn update_interval(&self) -> usize {
        return usize::max_value();
    }

    fn update_now(&self) -> bool {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        return self.borrow().update_now;
    }

    fn disable_update_now(&self) {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        self.borrow_mut().update_now = false;
    }

    fn parse_body(&mut self, text: &str) {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        self.borrow_mut().cache_items(text);
    }

    fn headers(&mut self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert("content-type", "application/json".parse().unwrap());
        h.insert("user-agent","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36".parse().unwrap());
        h.insert("accept", "*/*".parse().unwrap());

        h
    }
}

impl httpclient::PostContentProvider for QBox<Model> {
    fn content(&mut self) -> String {
        r#"{"volumeType":"eth","timePeriod":"twenty_four_hours"}"#.to_string()
    }
}

impl Model {
    pub fn init(&mut self) {
        self.url = "https://genie-production-api.herokuapp.com/collections/trending".to_string();
        self.async_update_model();
    }

    fn refresh_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        self.update_now = true;
    }

    fn new_item(raw_item: &RawItem) -> Item {
        return Item {
            name: raw_item.name.clone().into(),
            address: raw_item.address.clone().into(),
            percent_listed: raw_item.percent_listed.clone().into(),
            volume: raw_item.volume,
            volume_change: raw_item.volume_change,
            floor: raw_item.floor,
            market_cap: raw_item.market_cap,
            owners: raw_item.owners,
            supply: raw_item.supply,
        };
    }

    fn update_model(&mut self, _text: String) {
        {
            let _ = self.mutex.lock().unwrap();
            self.clear();
            let qptr = QBox::new(self);
            for item in &qptr.borrow().tmp_items {
                self.append(item.clone());
            }
        }

        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
    }

    fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::post(qptr, 1, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<Vec<RawItem>>(text) {
            Ok(raw_item) => {
                if raw_item.is_empty() {
                    return;
                }

                let mut bull_count = 0;
                let mut bear_count = 0;
                self.tmp_items.clear();

                for item in raw_item.iter() {
                    if item.volume_change > 0.0 {
                        bull_count += 1;
                    } else {
                        bear_count += 1;
                    }

                    self.tmp_items.push(Self::new_item(&item));
                }

                if bear_count <= 0 && bull_count <= 0 {
                    return;
                }

                self.bull_percent = bull_count as f32 / (bull_count + bear_count) as f32;
                self.bull_percent_changed();
            }
            Err(e) => debug!("{:?}", e),
        }
    }
}
