use super::data::{ProtocolItem as Item, RawProtocolItem as RawItem};
use super::sort::{ProtocolSortKey as SortKey, SortDir};
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

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        path: String,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
        tmp_items: ItemVec,
    }, members_qt: {
        bull_percent: [f32; bull_percent_changed], // 上涨占比
        update_now: [bool; update_now_changed], // 马上更新
        update_time: [QString; update_time_changed],// 数据更新时间
    }, signals_qt: {
    }, methods_qt: {
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
        self.borrow_mut().save(text);
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

        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.sort_key = SortKey::Index as u32;
        self.update_now = false;
        self.url = "https://api.llama.fi/protocols".to_string();

        self.path = app_dirs
            .data_dir
            .join("chain-protocols.json")
            .to_str()
            .unwrap()
            .to_string();

        self.async_update_model();
    }

    // 缓存数据到本地
    fn save(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.path, text) {
            warn!("write to {} error", &self.path);
        }
    }

    // 更新model
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
    pub fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 5, cb);
    }

    fn cache_items(&mut self, text: &str) {
        let raw_item: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);

        if raw_item.is_empty() {
            return;
        }

        let mut bull_count = 0;
        let mut bear_count = 0;

        self.tmp_items.clear();
        for (i, item) in raw_item.iter().enumerate() {
            if i >= 100 {
                break;
            }

            if item.change_1d.unwrap_or(0.0) > 0.0 {
                bull_count += 1;
            } else {
                bear_count += 1;
            }

            let mut item = Self::new(&item);
            item.index = i as i32;
            self.tmp_items.push(item);
        }

        self.bull_percent = bull_count as f32 / (bull_count + bear_count) as f32;
        self.bull_percent_changed();
    }

    fn toggle_sort_dir_qml(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    // 跟据key进行搜索
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

    // 生成一个新条目
    fn new(raw_item: &RawItem) -> Item {
        return Item {
            name: raw_item.name.clone().into(),
            symbol: raw_item.symbol.clone().into(),
            tvl: raw_item.tvl,
            market_cap_usd: raw_item.mcap,
            staking: raw_item.staking,
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
