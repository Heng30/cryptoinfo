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
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering as AOrdering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
        page: AtomicU32,
        update_now: AtomicBool,
    }, members_qt: {
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
        return self.borrow().update_now.load(AOrdering::SeqCst);
    }

    fn disable_update_now(&self) {
        self.borrow().update_now.store(false, AOrdering::SeqCst);
    }

    fn parse_body(&mut self, text: &str) {
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(cstr!("MonitorEthSortKey"), 1, 0, cstr!("MonitorEthSortKey"));
        self.sort_key = SortKey::TxValue as u32;
        self.page = AtomicU32::new(1);
        self.url = "https://api.yitaifang.com/index/largetxs/?page=".to_string();
        self.async_update_model();
    }

    fn get_page(&self) -> u32 {
        return self.page.load(AOrdering::SeqCst);
    }

    fn up_refresh_qml(&mut self) {
        self.page.store(1, AOrdering::SeqCst);
        self.update_now.store(true, AOrdering::SeqCst);
    }

    fn down_refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
    }

    fn new_item(raw_item: MonitorEthDataResultRawItem) -> Item {
        return Item {
            tx_hash: raw_item.tx.into(),
            blocktime: Utility::utc_seconds_to_local_string(raw_item.timestamp, "%Y-%m-%d %H:%M")
                .into(),
            from: raw_item.from.into(),
            to: raw_item.to.into(),
            tx_value: raw_item.amount.parse::<f64>().unwrap_or(-1.0)
                / raw_item.price.parse::<f64>().unwrap_or(-1.0),
        };
    }

    fn update_model(&mut self, _text: String) {
        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }
        if self.page.fetch_add(1, AOrdering::SeqCst) == 1 {
            self.clear();
        }

        for item in tmp_items.unwrap() {
            if self.items().contains(&item) {
                continue;
            }
            self.append(item);
        }

        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.refresh_ok();
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

                let mut v = vec![];
                for item in raw_item.data.result {
                    let amount = item.amount.parse().unwrap_or(-1.0);
                    let price = item.price.parse().unwrap_or(-1.0);
                    if amount <= 0.0 || price <= 0.0 || amount / price < 100.0_f64 {
                        continue;
                    }
                    v.push(Self::new_item(item));
                }
                *self.tmp_items.lock().unwrap() = Some(v);
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
