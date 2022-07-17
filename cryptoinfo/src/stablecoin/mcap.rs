use super::data::{StableCoinAssetRawItem, StableCoinRawItem as RawItem};
use super::sort::{McapSortKey as SortKey, SortDir};
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use qmetaobject::*;
use std::cmp::Ordering;
use StableCoinMcapItem as Item;

type ItemVec = Vec<Item>;

#[derive(QGadget, Clone, Default)]
pub struct StableCoinMcapItem {
    index: qt_property!(u32),
    name: qt_property!(QString),
    symbol: qt_property!(QString),
    peg_type: qt_property!(QString),
    price_source: qt_property!(QString),
    circulating: qt_property!(f64),
    price: qt_property!(f64),
}

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
    }, members_qt: {
        bull_percent: [f32; bull_percent_changed],
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
            cstr!("StableCoinMcapSortKey"),
            1,
            0,
            cstr!("StableCoinMcapSortKey"),
        );
        self.sort_key = SortKey::Circulating as u32;
        self.url = " https://stablecoins.llama.fi/stablecoins?includePrices=true".to_string();
        self.async_update_model();
    }

    fn new_item(raw_item: &StableCoinAssetRawItem) -> Item {
        return Item {
            index: 0,
            name: raw_item.name.clone().into(),
            symbol: raw_item.symbol.clone().into(),
            peg_type: raw_item.peg_type.clone().into(),
            circulating: raw_item.circulating.usd,
            price: raw_item.price.unwrap_or(-1f64),
            price_source: if raw_item.price_source.is_none() {
                "-".to_string().into()
            } else {
                raw_item.price_source.as_ref().unwrap().clone().into()
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

    // 更新数据
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
                if raw_item.pegged_assets.is_empty() {
                    return;
                }

                let mut bull_count = 0;
                let mut bear_count = 0;
                self.tmp_items.clear();

                for item in raw_item.pegged_assets.iter() {
                    if item.peg_type != "peggedUSD".to_string() {
                        continue;
                    }

                    if item.price > Some(1_f64) {
                        bull_count += 1;
                    } else {
                        bear_count += 1;
                    }
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
