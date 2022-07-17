use super::data::{MonitorBtcDataHitRawItem, MonitorBtcRawItem as RawItem};
use super::sort::{SortDir, SortKey};
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use qmetaobject::*;
use std::cmp::Ordering;
use MonitorBtcItem as Item;

type ItemVec = Vec<Item>;

#[derive(QGadget, Clone, Default)]
pub struct MonitorBtcItem {
    tx_hash: qt_property!(QString),
    blocktime: qt_property!(QString),
    from: qt_property!(QString),
    to: qt_property!(QString),
    tx_value: qt_property!(f64),
}

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
    }, members_qt: {
        update_now: [bool; update_now_changed],
        update_time: [QString; update_time_changed],
        total_tx_value:[f64; total_tx_value_changed],
    }, signals_qt: {
    }, methods_qt: {
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow().url.clone();
    }

    fn update_interval(&self) -> usize {
        return 600;
    }

    fn update_now(&self) -> bool {
        return self.borrow().update_now;
    }

    fn disable_update_now(&self) {
        self.borrow_mut().update_now = false;
    }

    fn parse_body(&mut self, text: &str) {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(
            cstr!("MonitorBtcSortKey"),
            1,
            0,
            cstr!("MonitorBtcSortKey"),
        );
        self.sort_key = SortKey::TxValue as u32;
        self.url = "https://api.btc126.vip/oklink.php?from=transfer".to_string();
        self.async_update_model();
    }


    fn new_item(raw_item: &MonitorBtcDataHitRawItem) -> Item {
        return Item {
            tx_hash: raw_item.tx_hash.clone().into(),
            blocktime: raw_item.blocktime.clone().into(),
            from: raw_item.from.clone().into(),
            to: raw_item.to.clone().into(),
            tx_value: raw_item.tx_value,
        };
    }

    fn update_model(&mut self, _text: String) {
        {
            let _ = self.mutex.lock().unwrap();
            let qptr = QBox::new(self);
            for (i, item) in qptr.borrow().tmp_items.iter().enumerate() {
                if self.items_len() > i {
                    self.set(i, item.clone());
                } else {
                    self.append(item.clone());
                }
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

        httpclient::download_timer_pro(qptr, 5, cb);
    }

    fn cache_items(&mut self, text: &str) {
        if let Ok(raw_item) = serde_json::from_str::<RawItem>(text) {
            if raw_item.data.hits.is_empty() {
                return;
            }
            self.tmp_items.clear();

            for item in raw_item.data.hits.iter() {
                self.tmp_items.push(Self::new_item(&item));
            }

            self.total_tx_value = raw_item.data.total;
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
            self.items_mut()
                .sort_by(|a, b| a.tx_value.partial_cmp(&b.tx_value).unwrap_or(Ordering::Less));
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
