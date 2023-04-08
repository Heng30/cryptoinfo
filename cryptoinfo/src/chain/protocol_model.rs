use super::data::{ProtocolItem as Item, RawProtocolItem as RawItem};
use super::sort::{ProtocolSortKey as SortKey, SortDir};
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
        bull_percent: [f32; bull_percent_changed], // 上涨占比
        update_time: [QString; update_time_changed],// 数据更新时间
    }, signals_qt: {
    }, methods_qt: {
        refresh_qml: fn(&mut self),
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
        search_and_view_at_beginning_qml: fn(&mut self, text: QString),
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
            cstr!("ChainProtocolSortKey"),
            1,
            0,
            cstr!("ChainProtocolSortKey"),
        );

        self.sort_key = SortKey::Index as u32;
        self.url = "https://api.llama.fi/protocols".to_string();
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

    pub fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 5, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<Vec<RawItem>>(text) {
            Ok(raw_item) => {
                let mut bull_count = 0;
                let mut bear_count = 0;
                let mut v = vec![];

                for (i, item) in raw_item.into_iter().enumerate() {
                    if i >= 100 {
                        break;
                    }

                    if item.change_1d.unwrap_or(0.0) > 0.0 {
                        bull_count += 1;
                    } else {
                        bear_count += 1;
                    }

                    let mut item = Self::new(item);
                    item.index = i as i32;
                    v.push(item);
                }
                *self.tmp_items.lock().unwrap() = Some(v);

                if bull_count + bear_count > 0 {
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
        if key == SortKey::Symbol {
            self.items_mut().sort_by(|a, b| {
                a.symbol
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.symbol.to_string().to_lowercase())
            });
        } else if key == SortKey::Name {
            self.items_mut().sort_by(|a, b| {
                a.name
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.name.to_string().to_lowercase())
            });
        } else if key == SortKey::Index {
            self.items_mut().sort_by(|a, b| a.index.cmp(&b.index));
        } else if key == SortKey::Per1H {
            self.items_mut().sort_by(|a, b| {
                a.percent_change_1h
                    .partial_cmp(&b.percent_change_1h)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Per24H {
            self.items_mut().sort_by(|a, b| {
                a.percent_change_24h
                    .partial_cmp(&b.percent_change_24h)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Per7D {
            self.items_mut().sort_by(|a, b| {
                a.percent_change_7d
                    .partial_cmp(&b.percent_change_7d)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::MarketCap {
            self.items_mut().sort_by(|a, b| {
                a.market_cap_usd
                    .partial_cmp(&b.market_cap_usd)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Staking {
            self.items_mut()
                .sort_by(|a, b| a.staking.partial_cmp(&b.staking).unwrap_or(Ordering::Less));
        } else if key == SortKey::TVL {
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

    #[allow(clippy::new_ret_no_self)]
    fn new(raw_item: RawItem) -> Item {
        return Item {
            name: raw_item.name.into(),
            symbol: raw_item.symbol.into(),
            tvl: raw_item.tvl.unwrap_or(0.0),
            market_cap_usd: raw_item.mcap.unwrap_or(0.0),
            staking: raw_item.staking.unwrap_or(0.0),
            percent_change_1h: raw_item.change_1h.unwrap_or(0.0),
            percent_change_24h: raw_item.change_1d.unwrap_or(0.0),
            percent_change_7d: raw_item.change_7d.unwrap_or(0.0),
            ..Default::default()
        };
    }

    // 查找并与第一行交换
    fn search_and_view_at_beginning_qml(&mut self, text: QString) {
        if let Some(index) = self
            .items()
            .iter()
            .position(|a| a.symbol.to_lower() == text.to_lower())
        {
            self.swap_row(0, index);
        }
    }
}
