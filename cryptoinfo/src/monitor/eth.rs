use super::data::{MonitorEthDataResultRawItem, MonitorEthRawItem as RawItem, MonitorItem as Item};
use super::sort::{SortDir, SortKey};
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use qmetaobject::*;
use std::cmp::Ordering;

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
        page: u32,
    }, members_qt: {
        update_now: [bool; update_now_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
        refresh_ok,
    }, methods_qt: {
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
        up_refresh_qml: fn(&mut self),
        down_refresh_qml: fn(&mut self),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return format!("{}{}", &self.borrow().url, self.borrow().get_page());
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
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(cstr!("MonitorEthSortKey"), 1, 0, cstr!("MonitorEthSortKey"));
        self.sort_key = SortKey::TxValue as u32;
        self.page = 1;
        self.url = "https://api.yitaifang.com/index/largetxs/?page=".to_string();
        self.async_update_model();
    }

    fn get_page(&self) -> u32 {
        let _ = self.mutex.lock().unwrap();
        return self.page;
    }

    fn up_refresh_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        self.page = 1;
        self.update_now = true;
    }

    fn down_refresh_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        self.update_now = true;
    }

    fn new_item(raw_item: &MonitorEthDataResultRawItem) -> Item {
        return Item {
            tx_hash: raw_item.tx.clone().into(),
            blocktime: Utility::utc_seconds_to_local_string(raw_item.timestamp, "%Y-%m-%d %H:%M")
                .clone()
                .into(),
            from: raw_item.from.clone().into(),
            to: raw_item.to.clone().into(),
            tx_value: raw_item.amount.parse::<f64>().unwrap_or(-1.0)
                / raw_item.price.parse::<f64>().unwrap_or(-1.0),
        };
    }

    fn update_model(&mut self, _text: String) {
        {
            let _ = self.mutex.lock().unwrap();
            if self.page == 1 {
                self.clear();
            }
            let qptr = QBox::new(self);
            for item in qptr.borrow().tmp_items.iter() {
                if self.items().contains(&item) {
                    continue;
                }
                self.append(item.clone());
            }
            self.page += 1;
        }

        self.refresh_ok();
        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
    }

    fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 5, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<RawItem>(text) {
            Ok(raw_item) => {
                if raw_item.data.result.is_empty() {
                    return;
                }
                self.tmp_items.clear();

                for item in raw_item.data.result.iter() {
                    let amount = item.amount.parse().unwrap_or(-1.0);
                    let price = item.price.parse().unwrap_or(-1.0);
                    if amount <= 0.0 || price <= 0.0 || amount / price < 100.0_f64 {
                        continue;
                    }
                    self.tmp_items.push(Self::new_item(&item));
                }
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
        if key == SortKey::TxValue {
            self.items_mut().sort_by(|a, b| {
                a.tx_value
                    .partial_cmp(&b.tx_value)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::BlockTime {
            self.items_mut()
                .sort_by(|a, b| a.blocktime.to_string().cmp(&b.blocktime.to_string()));
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
