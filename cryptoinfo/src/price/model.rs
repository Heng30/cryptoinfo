use super::data::{PriceItem as Item, Private, RawItem};
use super::sort::{SortDir, SortKey};
use crate::config::Config;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use std::cmp::Ordering;
use std::fs;

type PrivateVec = Vec<Private>;

modeldata_struct!(Model, Item, {
        price_path: String, // 缓存文件路径
        private_path: String, // 私有数据
        private: PrivateVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
    }, {
        bull_percent: [f32; bull_percent_changed],
        text: [QString; text_changed],
        update_interval: [u32; update_interval_changed], // 更新时间间隔
        update_now: [bool; update_now_changed], // 马上更新
        item_max_count: [u32; item_max_count_changed],
        update_time: [QString; update_time_changed], // 数据更新时间
    }, {
        set_url: fn(&mut self, limit: u32),
        set_marked: fn(&mut self, index: usize, marked: bool),
        set_floor_price: fn(&mut self, index: usize, price: f32),
        search_and_view_at_beginning: fn(&mut self, text: QString),
        sort_by_key: fn(&mut self, key: u32),
        toggle_sort_dir: fn(&mut self),
        update_all: fn(&mut self),
    }
);

impl Model {
    // 设置默认值
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(cstr!("PriceSortKey"), 1, 0, cstr!("PriceSortKey"));

        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        let config = qobj::<Config>(QNodeType::CONFIG);
        self.sort_key = SortKey::Marked as u32;
        self.update_interval = config.price_refresh_interval;
        self.update_now = false;
        self.set_url(config.price_item_count);

        let file = app_dirs.data_dir.join("private.json");
        self.private_path = file.to_str().unwrap().to_string();
        let file = app_dirs.data_dir.join("price.json");
        self.price_path = file.to_str().unwrap().to_string();

        self.load_private();
        self.load();
    }

    // 设置数据url
    fn set_url(&mut self, limit: u32) {
        self.item_max_count = limit;
        self.url = "https://api.alternative.me/v1/ticker/?limit=".to_string() + &limit.to_string();
    }

    // 价值私有数据
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

    // 加载本地缓存数据
    fn load(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.price_path) {
            if text.is_empty() {
                return;
            }

            self.reset(&text);
            self.sort_by_key(self.sort_key);
        }
    }

    // 缓存数据到本地
    fn save(&self, text: &str) {
        let tmp_path = self.private_path.clone() + ".tmp";
        if let Err(_) = std::fs::write(&tmp_path, text) {
            warn!("write to {} error", &tmp_path);
            return;
        }
        if fs::rename(&tmp_path, &self.price_path).is_err() {
            warn!("write to {} error", &self.price_path);
        }
    }

    // 更新model
    fn update_all(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.to_string().clone();
        self.reset(&text);
        self.save(&text);
        self.sort_by_key(self.sort_key);
        self.update_time = utility::Utility::default().local_time_now(QString::from("%H:%M:%S"));
        self.update_time_changed();
    }

    // 更新数据
    pub fn update_text(&mut self, text: String) {
        self.text = text.into();
        self.text_changed();
    }

    // 设置反向搜索
    fn toggle_sort_dir(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    // 跟据key进行搜索
    fn sort_by_key(&mut self, key: u32) {
        if self.items_is_empty() {
            return;
        }

        let key: SortKey = key.into();
        match self.sort_dir {
            SortDir::UP => {
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
                    self.items_mut().sort_by(|a, b| a.index.cmp(&b.index));
                    self.items_mut().sort_by(|a, b| a.marked.cmp(&b.marked));
                } else {
                    return;
                }
            }
            _ => {
                if key == SortKey::Symbol {
                    self.items_mut()
                        .sort_by(|a, b| b.symbol.to_string().cmp(&a.symbol.to_string()));
                } else if key == SortKey::Index {
                    self.items_mut().sort_by(|a, b| b.index.cmp(&a.index));
                } else if key == SortKey::Per24H {
                    self.items_mut().sort_by(|a, b| {
                        b.percent_change_24h
                            .partial_cmp(&a.percent_change_24h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Per7D {
                    self.items_mut().sort_by(|a, b| {
                        b.percent_change_7d
                            .partial_cmp(&a.percent_change_7d)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Volume24H {
                    self.items_mut().sort_by(|a, b| {
                        b.volume_24h_usd
                            .partial_cmp(&a.volume_24h_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Price {
                    self.items_mut().sort_by(|a, b| {
                        b.price_usd
                            .partial_cmp(&a.price_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Floor {
                    self.items_mut().sort_by(|a, b| {
                        b.floor_price
                            .partial_cmp(&a.floor_price)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Marked {
                    self.items_mut().sort_by(|a, b| a.index.cmp(&b.index));
                    self.items_mut().sort_by(|a, b| b.marked.cmp(&a.marked));
                } else {
                    return;
                }
            }
        }

        self.sort_key = key as u32;
        self.data_changed(0, self.items_len() - 1);
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
    fn set_marked(&mut self, index: usize, marked: bool) {
        if index >= self.items_len() {
            return;
        }

        let mut item = self.items()[index].clone();
        item.marked = marked;
        self.set(index, item);
        self.save_private();
    }

    // 设置地板价格
    fn set_floor_price(&mut self, index: usize, price: f32) {
        if index >= self.items_len() {
            return;
        }

        let mut item = self.items()[index].clone();
        item.floor_price = price;
        self.set(index, item);
        self.save_private();
    }

    // 添加条目
    fn add_item(&mut self, index: usize, raw_prices: &RawItem) {
        let mut item = Self::new_price(&raw_prices);
        item.index = index as i32;
        if let Some(pdata) = self.get_private(&raw_prices.symbol) {
            item.marked = pdata.marked;
            item.floor_price = pdata.floor_price;
        }
        self.append(item);
    }

    // 修改条目
    fn set_item(&mut self, index: usize, raw_prices: &RawItem) {
        let mut price = Self::new_price(&raw_prices);
        price.index = index as i32;
        if let Some(pdata) = self.get_private(&raw_prices.symbol) {
            price.marked = pdata.marked;
            price.floor_price = pdata.floor_price;
        }
        self.set(index, price);
    }

    // 条目不知列表中，则添加，在列表中则修改
    fn reset(&mut self, text: &str) {
        let raw_prices: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);
        let mut bull_count = 0;
        let mut bear_count = 0;

        for (i, item) in raw_prices.iter().enumerate() {
            if i > self.item_max_count as usize {
                break;
            }

            if item.percent_change_24h.parse().unwrap_or(0.0) > 0.0 {
                bull_count += 1;
            } else {
                bear_count += 1;
            }

            if self.items_len() <= i {
                self.add_item(i, &item);
            } else {
                self.set_item(i, &item);
            }
        }

        self.bull_percent = bull_count as f32 / (bull_count + bear_count) as f32;
        self.bull_percent_changed();
    }

    // 查找并与第一行交换
    fn search_and_view_at_beginning(&mut self, text: QString) {
        if let Some(index) = self
            .items()
            .iter()
            .position(|a| a.symbol.to_lower() == text.to_lower())
        {
            self.swap_row(0, index);
        }
    }
}
