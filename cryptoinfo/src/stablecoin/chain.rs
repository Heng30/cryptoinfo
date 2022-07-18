use super::data::StableCoinChainRawItem as RawItem;
use super::sort::{ChainSortKey as SortKey, SortDir};
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use qmetaobject::*;
use std::cmp::Ordering;
use StableCoinChainItem as Item;

type ItemVec = Vec<Item>;

#[derive(QGadget, Clone, Default)]
pub struct StableCoinChainItem {
    index: qt_property!(u32),
    name: qt_property!(QString),
    symbol: qt_property!(QString),
    circulating: qt_property!(f64),
}

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
    }, members_qt: {
        update_now: [bool; update_now_changed],
        update_time: [QString; update_time_changed],
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
        return 1800;
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
            cstr!("StableCoinChainSortKey"),
            1,
            0,
            cstr!("StableCoinChainSortKey"),
        );
        self.sort_key = SortKey::Circulating as u32;
        self.url = "https://stablecoins.llama.fi/stablecoinchains".to_string();
        self.async_update_model();
    }

    fn new_item(raw_item: &RawItem) -> Item {
        return Item {
            index: 0,
            name: raw_item.name.clone().into(),
            circulating: raw_item.circulating.usd,
            symbol: if raw_item.symbol.is_none() {
                "-".to_string().into()
            } else {
                raw_item.symbol.as_ref().unwrap().clone().into()
            },
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

        httpclient::download_timer_pro(qptr, 1, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<Vec<RawItem>>(text) {
            Ok(raw_item) => {
                self.tmp_items.clear();

                for item in raw_item.iter() {
                    self.tmp_items.push(Self::new_item(&item));
                }

                self.tmp_items.sort_by(|a, b| {
                    b.circulating
                        .partial_cmp(&a.circulating)
                        .unwrap_or(Ordering::Less)
                });

                for (i, mut item) in self.tmp_items.iter_mut().enumerate() {
                    item.index = i as u32;
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
        if key == SortKey::Name {
            self.items_mut()
                .sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));
        } else if key == SortKey::Index {
            self.items_mut().sort_by(|a, b| a.index.cmp(&b.index));
        } else if key == SortKey::Symbol {
            self.items_mut()
                .sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
        } else if key == SortKey::Circulating {
            self.items_mut().sort_by(|a, b| {
                a.circulating
                    .partial_cmp(&b.circulating)
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
