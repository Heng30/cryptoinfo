use super::data::StableCoinMcapItem as Item;
use super::data::{StableCoinAssetRawItem, StableCoinRawItem as RawItem};
use super::sort::{McapSortKey as SortKey, SortDir};
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
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
        update_now: AtomicBool,
    }, members_qt: {
        bull_percent: [f32; bull_percent_changed],
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
        qml_register_enum::<SortKey>(
            cstr!("StableCoinMcapSortKey"),
            1,
            0,
            cstr!("StableCoinMcapSortKey"),
        );
        self.sort_key = SortKey::Circulating as u32;
        self.url = "https://stablecoins.llama.fi/stablecoins?includePrices=true".to_string();
        self.async_update_model();
    }

    fn new_item(raw_item: StableCoinAssetRawItem) -> Item {
        return Item {
            index: 0,
            name: raw_item.name.into(),
            symbol: raw_item.symbol.into(),
            peg_type: raw_item.peg_type.into(),
            circulating: raw_item.circulating.usd,
            price: raw_item.price.unwrap_or(-1f64),
            price_source: match raw_item.price_source {
                None => "-".to_string().into(),
                Some(price_source) => price_source.into(),
            },
        };
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

        httpclient::download_timer_pro(qptr, 5, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<RawItem>(text) {
            Ok(raw_item) => {
                let mut bull_count = 0;
                let mut bear_count = 0;
                let mut v = vec![];

                for item in raw_item.pegged_assets {
                    if item.peg_type != *"peggedUSD" {
                        continue;
                    }

                    if item.price > Some(1_f64) {
                        bull_count += 1;
                    } else {
                        bear_count += 1;
                    }
                    v.push(Self::new_item(item));
                }

                v.sort_by(|a, b| {
                    b.circulating
                        .partial_cmp(&a.circulating)
                        .unwrap_or(Ordering::Less)
                });

                for (i, mut item) in v.iter_mut().enumerate() {
                    item.index = i as u32;
                }
                *self.tmp_items.lock().unwrap() = Some(v);

                if bear_count + bull_count > 0 {
                    self.bull_percent = bull_count as f32 / (bull_count + bear_count) as f32;
                    self.bull_percent_changed();
                }
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
        if key == SortKey::Name {
            self.items_mut()
                .sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));
        } else if key == SortKey::Source {
            self.items_mut()
                .sort_by(|a, b| a.price_source.to_string().cmp(&b.price_source.to_string()));
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
        } else if key == SortKey::Price {
            self.items_mut()
                .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Less));
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
