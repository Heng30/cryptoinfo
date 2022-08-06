use super::data::{PriceItem as Item, Private, RawItem};
use super::sort::{SortDir, SortKey};
use crate::config::{Config, PanelType};
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

type PrivateVec = Vec<Private>;
type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        price_path: String, // 缓存文件路径
        private_path: String, // 私有数据
        private: PrivateVec,
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
    }, members_qt:{
        bull_percent: [f32; bull_percent_changed],
        update_interval: [u32; update_interval_changed], // 更新时间间隔
        update_now: [bool; update_now_changed], // 马上更新
        item_max_count: [u32; item_max_count_changed],
        update_time: [QString; update_time_changed], // 数据更新时间
    }, signals_qt: {
    },
    methods_qt: {
        set_url_qml: fn(&mut self, limit: u32),
        set_marked_qml: fn(&mut self, index: usize, marked: bool),
        set_floor_price_qml: fn(&mut self, index: usize, price: f32),
        search_and_view_at_beginning_qml: fn(&mut self, text: QString),
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow().url.clone();
    }

    fn update_interval(&self) -> usize {
        return self.borrow().update_interval as usize;
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
        let conf = qobj::<Config>(QNodeType::Config);
        if conf.unrefresh_when_not_focus && conf.panel_type != PanelType::Price as u32 {
            return;
        }
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(cstr!("PriceSortKey"), 1, 0, cstr!("PriceSortKey"));

        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        let config = qobj::<Config>(QNodeType::Config);
        self.sort_key = SortKey::Marked as u32;
        self.update_interval = config.price_refresh_interval;
        self.update_now = false;
        self.set_url_qml(config.price_item_count);

        let file = app_dirs.data_dir.join("private.json");
        self.private_path = file.to_str().unwrap().to_string();
        let file = app_dirs.data_dir.join("price.json");
        self.price_path = file.to_str().unwrap().to_string();

        self.load_private();
        self.async_update_model();
    }

    // 加载私有数据
    fn load_private(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.private_path) {
            if let Ok(data) = serde_json::from_str::<PrivateVec>(&text) {
                self.private = data;
            }
        }
    }

    // 保存私有数据
    fn save_private(&mut self) {
        self.private.clear();
        for i in &self.inner_model.data {
            if !i.marked && i.floor_price < 0.00001 {
                continue;
            }
            self.private.push(Private {
                symbol: i.symbol.to_string(),
                marked: i.marked,
                floor_price: i.floor_price,
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&self.private) {
            if let Err(_) = std::fs::write(&self.private_path, text) {
                warn!("save {:?} failed", &self.private_path);
            }
        }
    }

    // 缓存数据到本地
    fn save(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.price_path, text) {
            warn!("write to {} error", &self.price_path);
        }
    }

    // 更新model
    fn update_model(&mut self, _text: String) {
        {
            let _ = self.mutex.lock().unwrap();
            if self.tmp_items.len() < self.items_len() {
                self.remove_rows(
                    self.tmp_items.len(),
                    self.items_len() - self.tmp_items.len(),
                );
            }

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

        httpclient::download_timer_pro(qptr, 1, cb);
    }

    // 条目不知列表中，则添加，在列表中则修改
    fn cache_items(&mut self, text: &str) {
        let raw_prices: Vec<RawItem> = serde_json::from_str(text).unwrap_or(vec![]);
        if raw_prices.is_empty() {
            return;
        }

        let mut bull_count = 0;
        let mut bear_count = 0;
        self.tmp_items.clear();

        for (i, item) in raw_prices.iter().enumerate() {
            if i >= self.item_max_count as usize {
                break;
            }

            if item.percent_change_24h.parse().unwrap_or(0.0) > 0.0 {
                bull_count += 1;
            } else {
                bear_count += 1;
            }

            let mut it = Self::new_price(&item);
            it.index = i as i32;
            if let Some(pdata) = self.get_private(&item.symbol) {
                it.marked = pdata.marked;
                it.floor_price = pdata.floor_price;
            }
            self.tmp_items.push(it);
        }

        if bear_count <= 0 && bull_count <= 0 {
            return;
        }

        self.bull_percent = bull_count as f32 / (bull_count + bear_count) as f32;
        self.bull_percent_changed();
    }

    // 跟据key进行搜索
    fn sort_by_key_qml(&mut self, key: u32) {
        if self.items_is_empty() {
            return;
        }

        let key: SortKey = key.into();
        if key == SortKey::Symbol {
            self.items_mut()
                .sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
        } else if key == SortKey::Index {
            self.items_mut().sort_by(|a, b| a.index.cmp(&b.index));
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
        } else if key == SortKey::Volume24H {
            self.items_mut().sort_by(|a, b| {
                a.volume_24h_usd
                    .partial_cmp(&b.volume_24h_usd)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Price {
            self.items_mut().sort_by(|a, b| {
                a.price_usd
                    .partial_cmp(&b.price_usd)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Floor {
            self.items_mut().sort_by(|a, b| {
                a.floor_price
                    .partial_cmp(&b.floor_price)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Marked {
            self.items_mut().sort_by(|b, a| a.index.cmp(&b.index));
            self.items_mut().sort_by(|a, b| a.marked.cmp(&b.marked));
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
    fn new_price(raw_prices: &RawItem) -> Item {
        return Item {
            id: raw_prices.id.clone().into(),
            name: raw_prices.name.clone().into(),
            symbol: raw_prices.symbol.clone().into(),
            rank: raw_prices.rank.parse().unwrap_or(0),
            price_usd: raw_prices.price_usd.parse().unwrap_or(0.0),
            volume_24h_usd: raw_prices.volume_24h_usd.parse().unwrap_or(0.0),
            market_cap_usd: raw_prices.market_cap_usd.parse().unwrap_or(0),
            available_supply: raw_prices.available_supply.parse().unwrap_or(0),
            total_supply: raw_prices.total_supply.parse().unwrap_or(0),
            max_supply: raw_prices.max_supply.parse().unwrap_or(0),
            percent_change_1h: raw_prices.percent_change_1h.parse().unwrap_or(0.0),
            percent_change_24h: raw_prices.percent_change_24h.parse().unwrap_or(0.0),
            percent_change_7d: raw_prices.percent_change_7d.parse().unwrap_or(0.0),
            last_updated: raw_prices.last_updated.parse().unwrap_or(0),
            ..Item::default()
        };
    }

    // 获取私有数据
    fn get_private(&self, symbol: &str) -> Option<&Private> {
        for item in &self.private {
            if item.symbol.to_lowercase() == symbol.to_lowercase() {
                return Some(item);
            }
        }
        return None;
    }

    // 设置关注
    fn set_marked_qml(&mut self, index: usize, marked: bool) {
        if index >= self.items_len() {
            return;
        }

        let mut item = self.items()[index].clone();
        item.marked = marked;
        self.set(index, item);
        self.save_private();
    }

    // 设置地板价格
    fn set_floor_price_qml(&mut self, index: usize, price: f32) {
        if index >= self.items_len() {
            return;
        }

        let mut item = self.items()[index].clone();
        item.floor_price = price;
        self.set(index, item);
        self.save_private();
    }

    // 设置数据url
    fn set_url_qml(&mut self, limit: u32) {
        self.item_max_count = limit;
        self.url = "https://api.alternative.me/v1/ticker/?limit=".to_string() + &limit.to_string();
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

    // 设置反向搜索
    fn toggle_sort_dir_qml(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }
}
