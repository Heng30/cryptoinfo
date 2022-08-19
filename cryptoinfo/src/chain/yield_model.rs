use super::data::{RawYieldDataItem, RawYieldItem as RawItem, YieldItem as Item};
use super::sort::{SortDir, YieldSortKey as SortKey};
use crate::httpclient;
use crate::utility::Utility;
use ::log::debug;
use cstr::cstr;
use modeldata::*;
use qmetaobject::*;
use std::cmp::Ordering;
use std::sync::atomic::{AtomicBool, Ordering as AOrdering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
        tmp_items: ItemVec,
        update_now: AtomicBool,
    }, members_qt: {
        update_time: [QString; update_time_changed],
    }, signals_qt: {
    }, methods_qt: {
        refresh_qml: fn(&mut self),
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
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
        qml_register_enum::<SortKey>(cstr!("ChainYieldSortKey"), 1, 0, cstr!("ChainYieldSortKey"));

        self.sort_key = SortKey::Index as u32;
        self.url = "https://yields.llama.fi/pools".to_string();
        self.async_update_model();
    }

    fn update_model(&mut self, _text: String) {
        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }

        self.clear();
        for item in tmp_items.unwrap() {
            self.append(item);
        }

        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.sort_by_key_qml(self.sort_key);
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
        match serde_json::from_str::<RawItem>(text) {
            Ok(mut raw_item) => {
                raw_item
                    .data
                    .sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(Ordering::Less));

                let mut v = vec![];
                for (i, item) in raw_item.data.into_iter().enumerate() {
                    if i >= 100 {
                        break;
                    }

                    let mut item = Self::new(&item);
                    item.index = i as i32;
                    v.push(item);
                }
                *self.tmp_items.lock().unwrap() = Some(v);
            }
            Err(e) => debug!("{:?}", e),
        }
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
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
        if key == SortKey::Symbol {
            self.items_mut().sort_by(|a, b| {
                a.symbol
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.symbol.to_string().to_lowercase())
            });
        } else if key == SortKey::Exposure {
            self.items_mut().sort_by(|a, b| {
                a.exposure
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.exposure.to_string().to_lowercase())
            });
        } else if key == SortKey::Pool {
            self.items_mut().sort_by(|a, b| {
                a.pool
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.pool.to_string().to_lowercase())
            });
        } else if key == SortKey::Project {
            self.items_mut().sort_by(|a, b| {
                a.project
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.project.to_string().to_lowercase())
            });
        } else if key == SortKey::Chain {
            self.items_mut().sort_by(|a, b| {
                a.chain
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.chain.to_string().to_lowercase())
            });
        } else if key == SortKey::Index {
            self.items_mut().sort_by(|a, b| a.index.cmp(&b.index));
        } else if key == SortKey::StableCoin {
            self.items_mut()
                .sort_by(|a, b| a.stablecoin.cmp(&b.stablecoin));
        } else if key == SortKey::Apy {
            self.items_mut()
                .sort_by(|a, b| a.apy.partial_cmp(&b.apy).unwrap_or(Ordering::Less));
        } else if key == SortKey::Tvl {
            self.items_mut()
                .sort_by(|a, b| a.tvl.partial_cmp(&b.tvl).unwrap_or(Ordering::Less));
        } else {
            return;
        }

        if self.sort_dir != SortDir::UP {
            self.items_mut().reverse();
        }
        self.sort_key = key as u32;
        self.items_changed(0, self.items_len() - 1);
    }

    fn new(raw_item: &RawYieldDataItem) -> Item {
        return Item {
            chain: raw_item.chain.clone().into(),
            project: raw_item.project.clone().into(),
            symbol: raw_item.symbol.clone().into(),
            apy: raw_item.apy,
            pool: raw_item.pool.clone().into(),
            stablecoin: raw_item.stablecoin,
            exposure: raw_item.exposure.clone().into(),
            tvl: raw_item.tvl,
            ..Default::default()
        };
    }
}
