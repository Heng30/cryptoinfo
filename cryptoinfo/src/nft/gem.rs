use super::data::{NFTGemDataRawItem, NFTGemRawItem as RawItem, NTFGemItem as Item};
use super::sort::{SortDir, SortKey};
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use qmetaobject::*;
use reqwest::header::HeaderMap;
use std::cmp::Ordering;

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
    }, members_qt: {
        bull_percent: [f32; bull_percent_changed],
        update_now: [bool; update_now_changed], // 马上更新
        update_time: [QString; update_time_changed], // 数据更新时间
    }, signals_qt: {
    }, methods_qt: {
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
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
        h.insert("origin", "https://www.gem.xyz".parse().unwrap());
        h.insert("referer", "https://www.gem.xyz/".parse().unwrap());
        h.insert(
            "x-api-key",
            "iMHRYlpIXs3zfcBY1r3iKLdqS2YUuOUs".parse().unwrap(),
        );
        h
    }
}

impl httpclient::PostContentProvider for QBox<Model> {
    fn content(&mut self) -> String {
        r#"{"sort":{"stats.one_day_volume":-1},"limit":100,"fields":{"name":1,"stats":1}}"#
            .to_string()
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(cstr!("NFTGemSortKey"), 1, 0, cstr!("NFTGemSortKey"));
        self.sort_key = SortKey::OneDayVolume as u32;

        self.url = "https://search.gemlabs.xyz/collections".to_string();
        self.async_update_model();
    }

    fn refresh_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        self.update_now = true;
    }

    fn new_item(raw_item: &NFTGemDataRawItem) -> Item {
        return Item {
            name: raw_item.name.clone().into(),
            one_day_volume: raw_item.stats.one_day_volume,
            one_day_change: raw_item.stats.one_day_change,
            seven_day_change: raw_item.stats.seven_day_change,
            total_volume: raw_item.stats.total_volume,
            total_sales: raw_item.stats.total_sales,
            total_supply: raw_item.stats.total_supply,
            num_owners: raw_item.stats.num_owners,
            floor_price: raw_item.stats.floor_price.unwrap_or(0_f64),
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

        self.sort_by_key_qml(self.sort_key);
        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
    }

    fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::post(qptr, 5, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<RawItem>(text) {
            Ok(raw_item) => {
                if raw_item.data.is_empty() {
                    return;
                }

                let mut bull_count = 0;
                let mut bear_count = 0;
                self.tmp_items.clear();

                for item in raw_item.data.iter() {
                    if item.stats.one_day_change > 0.0 {
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

    fn toggle_sort_dir_qml(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    fn sort_by_key_qml(&mut self, key: u32) {
        if self.items_is_empty() {
            return;
        }

        let key: SortKey = key.into();
        if key == SortKey::Name {
            self.items_mut().sort_by(|a, b| {
                a.name
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.name.to_string().to_lowercase())
            });
        } else if key == SortKey::OneDayVolume {
            self.items_mut().sort_by(|a, b| {
                a.one_day_volume
                    .partial_cmp(&b.one_day_volume)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::OneDayChange {
            self.items_mut().sort_by(|a, b| {
                a.one_day_change
                    .partial_cmp(&b.one_day_change)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::SevenDayChange {
            self.items_mut().sort_by(|a, b| {
                a.seven_day_change
                    .partial_cmp(&b.seven_day_change)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::TotalVolume {
            self.items_mut().sort_by(|a, b| {
                a.total_volume
                    .partial_cmp(&b.total_volume)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::TotalSales {
            self.items_mut().sort_by(|a, b| {
                a.total_sales
                    .partial_cmp(&b.total_sales)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::TotalSupply {
            self.items_mut().sort_by(|a, b| {
                a.total_supply
                    .partial_cmp(&b.total_supply)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::NumOwners {
            self.items_mut().sort_by(|a, b| {
                a.num_owners
                    .partial_cmp(&b.num_owners)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::FloorPrice {
            self.items_mut().sort_by(|a, b| {
                a.floor_price
                    .partial_cmp(&b.floor_price)
                    .unwrap_or(Ordering::Less)
            });
        } else {
            return;
        }

        if self.sort_dir != SortDir::UP {
            self.items_mut().reverse();
        }
        self.sort_key = key as u32;
        self.items_changed(0, self.items_len() - 1);
    }
}
