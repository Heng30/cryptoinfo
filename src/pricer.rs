use cstr::cstr;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::config::Config as conf;

/// 保存到本地的有关价格条目的私有数据
#[derive(Serialize, Deserialize, Debug)]
struct PrivateData {
    symbol: String,
    marked: bool,
    floor_price: f32,
}

/// 从json文件中解析出来的条目对象
#[derive(Serialize, Deserialize, Debug)]
struct RawPricer {
    id: String,
    name: String,
    symbol: String,
    rank: String,
    price_usd: String,
    market_cap_usd: String,
    available_supply: String,
    total_supply: String,
    max_supply: String,
    percent_change_1h: String,
    percent_change_24h: String,
    percent_change_7d: String,
    last_updated: String,

    #[serde(rename(serialize = "24h_volume_usd", deserialize = "24h_volume_usd"))]
    volume_24h_usd: String,
}

/// 与qml交互的条目对象
#[derive(QGadget, Clone, Default)]
struct Pricer {
    index: qt_property!(i32),
    marked: qt_property!(bool),
    floor_price: qt_property!(f32),

    id: qt_property!(QString),
    name: qt_property!(QString),
    symbol: qt_property!(QString),
    rank: qt_property!(u32),
    price_usd: qt_property!(f32),
    market_cap_usd: qt_property!(i64),
    available_supply: qt_property!(i64),
    total_supply: qt_property!(i64),
    max_supply: qt_property!(i64),
    percent_change_1h: qt_property!(f32),
    percent_change_24h: qt_property!(f32),
    percent_change_7d: qt_property!(f32),
    last_updated: qt_property!(i64),

    volume_24h_usd: qt_property!(f64),
}

/// 搜索方向
#[derive(Debug, PartialEq)]
enum SortDir {
    UP,
    DOWN,
}

impl Default for SortDir {
    fn default() -> Self {
        return SortDir::DOWN;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
enum SortKey {
    Marked = 1,
    MarketCap = 2,
    Symbol = 3,
    Price = 4,
    Per24H = 5,
    Per7D = 6,
    Volume24H = 7,
    Floor = 8,
}

impl From<u32> for SortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SortKey::Marked,
            2 => return SortKey::MarketCap,
            3 => return SortKey::Symbol,
            4 => return SortKey::Price,
            5 => return SortKey::Per24H,
            6 => return SortKey::Per7D,
            7 => return SortKey::Volume24H,
            8 => return SortKey::Floor,
            _ => return SortKey::Marked,
        }
    }
}

/// 与qml交互的model对象
#[derive(QObject, Default)]
pub struct Model {
    base: qt_base_class!(trait QAbstractListModel),
    data: Vec<Pricer>,
    count: qt_property!(i32; READ row_count NOTIFY count_changed),
    count_changed: qt_signal!(),

    bull_percent: qt_property!(f32; NOTIFY bull_percent_changed), // 上涨占比
    bull_percent_changed: qt_signal!(),

    set_marked: qt_method!(fn(&mut self, index: usize, marked: bool)),
    set_floor_price: qt_method!(fn(&mut self, index: usize, price: f32)),

    clear: qt_method!(fn(&mut self)),
    insert_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    remove_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    swap_row: qt_method!(fn(&mut self, from: usize, to: usize)),
    search_and_show_at_beginning: qt_method!(fn(&mut self, text: QString)),

    sort_by_key: qt_method!(fn(&mut self, key: u32)),
    toggle_sort_dir: qt_method!(fn(&mut self)),

    // 配置文件路径
    price_path: String,
    private_data_path: String,
    private_data: Vec<PrivateData>,

    // 排序相关
    sort_key: u32,
    sort_dir: SortDir,

    // 更新数据相关
    text: String,
    text_changed: qt_signal!(),
    update_all_price: qt_method!(fn(&mut self)),

    pub update_interval: qt_property!(u32; NOTIFY update_interval_changed), // 更新时间间隔
    update_interval_changed: qt_signal!(),

    pub update_now: qt_property!(bool; NOTIFY update_now_changed), // 马上更新
    update_now_changed: qt_signal!(),

    pub price_url: String,
    set_price_url: qt_method!(fn(&mut self, limit: u32)),
}

/// qml model要实现的接口
impl QAbstractListModel for Model {
    fn row_count(&self) -> i32 {
        self.data.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        if role != USER_ROLE {
            return QVariant::default();
        }

        self.data
            .get(index.row() as usize)
            .map(|x| x.to_qvariant())
            .unwrap_or_default()
    }

    fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {
        vec![(USER_ROLE, QByteArray::from("modelData"))]
            .into_iter()
            .collect()
    }
}

impl Model {
    pub fn init_from_engine(engine: &mut QmlEngine, model: QObjectPinned<Model>) {
        engine.set_object_property("pricer_model".into(), model);
        qml_register_enum::<SortKey>(cstr!("PriceSortKey"), 1, 0, cstr!("PriceSortKey"));
    }

    pub fn init_default(&mut self, config: &conf) {
        self.sort_key = SortKey::Marked as u32;
        self.update_interval = config.price_refresh_interval;
        self.update_now = false;
        self.price_url = "https://api.alternative.me/v1/ticker/?limit=".to_string()
            + &config.price_item_count.to_string();
    }

    pub fn set_private_data_path(&mut self, filepath: &str) {
        self.private_data_path = filepath.to_string();
    }

    fn set_price_url(&mut self, limit: u32) {
        self.price_url =
            "https://api.alternative.me/v1/ticker/?limit=".to_string() + &limit.to_string();
    }

    pub fn load_private_data(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.private_data_path) {
            if let Ok(data) = serde_json::from_str::<Vec<PrivateData>>(&text) {
                self.private_data = data;
            }
        }
    }

    fn save_private_data(&mut self) {
        self.private_data.clear();
        for i in &self.data {
            if !i.marked && i.floor_price < 0.00001 {
                continue;
            }
            self.private_data.push(PrivateData {
                symbol: i.symbol.to_string(),
                marked: i.marked,
                floor_price: i.floor_price,
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&self.private_data) {
            if let Err(_) = std::fs::write(&self.private_data_path, text) {
                warn!("save {:?} failed", &self.private_data_path);
            }
        }
    }

    pub fn set_price_path(&mut self, filepath: &str) {
        self.price_path = filepath.to_string();
    }

    pub fn load_prices(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.price_path) {
            if text.is_empty() {
                return;
            }

            self.reset(&text);
            self.sort_by_key(self.sort_key);
        }
    }

    fn save_prices(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.price_path, text) {
            warn!("write to {} error", &self.price_path);
        }
    }

    pub fn update_all_price(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.clone();
        self.reset(&text);
        self.save_prices(&text);
        self.sort_by_key(self.sort_key);
    }

    pub fn update_text(&mut self, text: String) {
        self.text = text;
        self.text_changed();
    }

    #[allow(dead_code)]
    pub fn fake_data() -> Self {
        return Self {
            data: vec![
                Pricer {
                    ..Default::default()
                },
                Pricer::default(),
            ],
            ..Default::default()
        };
    }

    /// 设置反向搜索
    fn toggle_sort_dir(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    /// 跟据key进行搜索
    fn sort_by_key(&mut self, key: u32) {
        if self.data.is_empty() {
            return;
        }

        let key: SortKey = key.into();
        match self.sort_dir {
            SortDir::UP => {
                if key == SortKey::Symbol {
                    self.data
                        .sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
                } else if key == SortKey::MarketCap {
                    self.data.sort_by(|a, b| a.index.cmp(&b.index));
                } else if key == SortKey::Per24H {
                    self.data.sort_by(|a, b| {
                        a.percent_change_24h
                            .partial_cmp(&b.percent_change_24h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Per7D {
                    self.data.sort_by(|a, b| {
                        a.percent_change_7d
                            .partial_cmp(&b.percent_change_7d)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Volume24H {
                    self.data.sort_by(|a, b| {
                        a.volume_24h_usd
                            .partial_cmp(&b.volume_24h_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Price {
                    self.data.sort_by(|a, b| {
                        a.price_usd
                            .partial_cmp(&b.price_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Floor {
                    self.data.sort_by(|a, b| {
                        a.floor_price
                            .partial_cmp(&b.floor_price)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Marked {
                    self.data.sort_by(|a, b| a.index.cmp(&b.index));
                    self.data.sort_by(|a, b| a.marked.cmp(&b.marked));
                } else {
                    return;
                }
            }
            _ => {
                if key == SortKey::Symbol {
                    self.data
                        .sort_by(|a, b| b.symbol.to_string().cmp(&a.symbol.to_string()));
                } else if key == SortKey::MarketCap {
                    self.data.sort_by(|a, b| b.index.cmp(&a.index));
                } else if key == SortKey::Per24H {
                    self.data.sort_by(|a, b| {
                        b.percent_change_24h
                            .partial_cmp(&a.percent_change_24h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Per7D {
                    self.data.sort_by(|a, b| {
                        b.percent_change_7d
                            .partial_cmp(&a.percent_change_7d)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Volume24H {
                    self.data.sort_by(|a, b| {
                        b.volume_24h_usd
                            .partial_cmp(&a.volume_24h_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Price {
                    self.data.sort_by(|a, b| {
                        b.price_usd
                            .partial_cmp(&a.price_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Floor {
                    self.data.sort_by(|a, b| {
                        b.floor_price
                            .partial_cmp(&a.floor_price)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Marked {
                    self.data.sort_by(|a, b| a.index.cmp(&b.index));
                    self.data.sort_by(|a, b| b.marked.cmp(&a.marked));
                } else {
                    return;
                }
            }
        }

        self.sort_key = key as u32;

        let end = self.data.len();
        let idx1 = (self as &mut dyn QAbstractListModel).row_index(0);
        let idx2 = (self as &mut dyn QAbstractListModel).row_index((end - 1) as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx1, idx2);
    }

    /// 生成一个新条目
    fn new_price(raw_prices: &RawPricer) -> Pricer {
        return Pricer {
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
            ..Pricer::default()
        };
    }

    fn get_private_data(&self, symbol: &str) -> Option<&PrivateData> {
        for item in &self.private_data {
            if item.symbol.to_lowercase() == symbol.to_lowercase() {
                return Some(item);
            }
        }
        return None;
    }

    fn set_marked(&mut self, index: usize, marked: bool) {
        if index >= self.data.len() {
            return;
        }

        self.data[index as usize].marked = marked;
        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
        self.save_private_data();
    }

    fn set_floor_price(&mut self, index: usize, price: f32) {
        if index >= self.data.len() {
            return;
        }

        self.data[index as usize].floor_price = price;
        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
        self.save_private_data();
    }

    /// 添加条目
    fn add(&mut self, index: usize, raw_prices: &RawPricer) {
        let end = self.data.len();
        (self as &mut dyn QAbstractListModel).begin_insert_rows(end as i32, end as i32);

        let mut price = Self::new_price(&raw_prices);
        price.index = index as i32;
        if let Some(pdata) = self.get_private_data(&raw_prices.symbol) {
            price.marked = pdata.marked;
            price.floor_price = pdata.floor_price;
        }

        self.data.insert(end, price);
        (self as &mut dyn QAbstractListModel).end_insert_rows();
        self.count_changed();
    }

    // 修改条目
    fn set(&mut self, index: usize, raw_prices: &RawPricer) {
        if index >= self.data.len() {
            return;
        }

        let mut price = Self::new_price(&raw_prices);
        price.index = index as i32;
        if let Some(pdata) = self.get_private_data(&raw_prices.symbol) {
            price.marked = pdata.marked;
            price.floor_price = pdata.floor_price;
        }
        self.data[index] = price;

        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
    }

    /// 条目不知列表中，则添加，在列表中则修改
    fn reset(&mut self, text: &str) {
        let raw_prices: Vec<RawPricer> = serde_json::from_str(&text).unwrap_or(vec![]);
        let mut bull_count = 0;
        let mut bear_count = 0;

        for (i, item) in raw_prices.iter().enumerate() {
            if item.percent_change_24h.parse().unwrap_or(0.0) > 0.0 {
                bull_count += 1;
            } else {
                bear_count += 1;
            }

            if self.data.len() <= i {
                self.add(i, &item);
            } else {
                self.set(i, &item);
            }
        }

        self.bull_percent = bull_count as f32 / (bull_count + bear_count) as f32;
        self.bull_percent_changed();
    }

    fn clear(&mut self) {
        (self as &mut dyn QAbstractListModel).begin_reset_model();
        self.data = vec![];
        (self as &mut dyn QAbstractListModel).end_reset_model();
        self.count_changed();
    }

    fn insert_rows(&mut self, row: usize, count: usize) -> bool {
        if count == 0 || row > self.data.len() {
            return false;
        }

        (self as &mut dyn QAbstractListModel)
            .begin_insert_rows(row as i32, (row + count - 1) as i32);
        for i in 0..count {
            self.data.insert(row + i, Pricer::default());
        }
        (self as &mut dyn QAbstractListModel).end_insert_rows();
        self.count_changed();
        true
    }

    fn remove_rows(&mut self, row: usize, count: usize) -> bool {
        if count == 0 || row + count > self.data.len() {
            return false;
        }
        (self as &mut dyn QAbstractListModel)
            .begin_remove_rows(row as i32, (row + count - 1) as i32);
        self.data.drain(row..row + count);
        (self as &mut dyn QAbstractListModel).end_remove_rows();
        self.count_changed();
        true
    }

    fn swap_row(&mut self, from: usize, to: usize) {
        if std::cmp::max(from, to) >= self.data.len() {
            return;
        }
        self.data.swap(from, to);

        let idx = (self as &mut dyn QAbstractListModel).row_index(from as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);

        let idx = (self as &mut dyn QAbstractListModel).row_index(to as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
    }

    fn search_and_show_at_beginning(&mut self, text: QString) {
        if let Some(index) = self
            .data
            .iter()
            .position(|a| a.symbol.to_lower() == text.to_lower())
        {
            self.swap_row(0, index);
        }
    }
}
