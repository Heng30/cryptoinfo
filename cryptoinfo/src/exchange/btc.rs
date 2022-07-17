use super::data::{ExchangeBtcDataExchangeRawItem, ExchangeBtcRawItem as RawItem};
use super::sort::{SortDir, SortKey};
use crate::httpclient;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use std::cmp::Ordering;
use ExchangeBtcItem as Item;

type ItemVec = Vec<Item>;

#[derive(QGadget, Clone, Default)]
pub struct ExchangeBtcItem {
    name: qt_property!(QString),
    balance: qt_property!(f64),
    income: qt_property!(f64),
    rate: qt_property!(f32),
}

modeldata_struct!(Model, Item, members: {
        path: String,
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
        self.borrow_mut().save(text);
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(
            cstr!("ExchangeBtcSortKey"),
            1,
            0,
            cstr!("ExchangeBtcSortKey"),
        );
        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        self.sort_key = SortKey::Balance as u32;
        self.url = "https://api.btc126.vip/blockinfo.php?from=exchagebtc".to_string();
        self.path = app_dirs
            .data_dir
            .join("exchange-btc.json")
            .to_str()
            .unwrap()
            .to_string();
        self.async_update_model();
    }

    fn save(&mut self, text: &str) {
        if let Err(_) = std::fs::write(&self.path, &text) {
            warn!("save {:?} failed", &self.path);
        }
    }

    fn new_item(raw_item: &ExchangeBtcDataExchangeRawItem) -> Item {
        return Item {
            name: raw_item.name.clone().into(),
            income: raw_item.income,
            rate: raw_item.rate,
            balance: raw_item.balance,
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

    // 更新数据
    fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 5, cb);
    }

    fn cache_items(&mut self, text: &str) {
        if let Ok(raw_item) = serde_json::from_str::<RawItem>(text) {
            if raw_item.data.exchanges.is_empty() {
                return;
            }

            let mut bull_count = 0;
            let mut bear_count = 0;
            self.tmp_items.clear();

            for item in raw_item.data.exchanges.iter() {
                if item.rate > 0.0 {
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
            self.items_mut()
                .sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));
        } else if key == SortKey::Balance {
            self.items_mut()
                .sort_by(|a, b| a.balance.partial_cmp(&b.balance).unwrap_or(Ordering::Less));
        } else if key == SortKey::Income {
            self.items_mut()
                .sort_by(|a, b| a.income.partial_cmp(&b.income).unwrap_or(Ordering::Less));
        } else if key == SortKey::Rate {
            self.items_mut()
                .sort_by(|a, b| a.rate.partial_cmp(&b.rate).unwrap_or(Ordering::Less));
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
