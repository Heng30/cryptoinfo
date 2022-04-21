use cstr::cstr;
use qmetaobject::*;
use std::cmp::Ordering;

#[allow(unused_imports)]
use ::log::{debug, warn};

use crate::config::Config as conf;
use crate::pricer::data::{PItem, Private, RawItem};
use crate::pricer::sort::{SortDir, SortKey};
use crate::utility;

/// 与qml交互的model对象
#[derive(QObject, Default)]
pub struct Model {
    base: qt_base_class!(trait QAbstractListModel),
    data: Vec<PItem>,
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
    private_data: Vec<Private>,

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

    // 数据更新时间
    update_time: qt_property!(QString; NOTIFY update_time_changed),
    update_time_changed: qt_signal!(),
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
    // 添加到qml环境
    pub fn init_from_engine(engine: &mut QmlEngine, model: QObjectPinned<Model>) {
        engine.set_object_property("pricer_model".into(), model);
        qml_register_enum::<SortKey>(cstr!("PriceSortKey"), 1, 0, cstr!("PriceSortKey"));
    }

    // 设置默认值
    pub fn init_default(&mut self, config: &conf) {
        self.sort_key = SortKey::Marked as u32;
        self.update_interval = config.price_refresh_interval;
        self.update_now = false;
        self.price_url = "https://api.alternative.me/v1/ticker/?limit=".to_string()
            + &config.price_item_count.to_string();
    }

    // 设置私有数据文件路径
    pub fn set_private_data_path(&mut self, filepath: &str) {
        self.private_data_path = filepath.to_string();
    }

    // 设置数据url
    fn set_price_url(&mut self, limit: u32) {
        self.price_url =
            "https://api.alternative.me/v1/ticker/?limit=".to_string() + &limit.to_string();
    }

    // 价值私有数据
    pub fn load_private_data(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.private_data_path) {
            if let Ok(data) = serde_json::from_str::<Vec<Private>>(&text) {
                self.private_data = data;
            }
        }
    }

    // 保存私有数据
    fn save_private_data(&mut self) {
        self.private_data.clear();
        for i in &self.data {
            if !i.marked && i.floor_price < 0.00001 {
                continue;
            }
            self.private_data.push(Private {
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

    // 设置缓存文件路径
    pub fn set_price_path(&mut self, filepath: &str) {
        self.price_path = filepath.to_string();
    }

    // 加载本地缓存数据
    pub fn load_prices(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.price_path) {
            if text.is_empty() {
                return;
            }

            self.reset(&text);
            self.sort_by_key(self.sort_key);
        }
    }

    // 缓存数据到本地
    fn save_prices(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.price_path, text) {
            warn!("write to {} error", &self.price_path);
        }
    }

    // 更新model
    fn update_all_price(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.clone();
        self.reset(&text);
        self.save_prices(&text);
        self.sort_by_key(self.sort_key);
        self.update_time = utility::Utility::default().local_time_now(QString::from("%H:%M:%S"));
        self.update_time_changed();
    }

    // 更新数据
    pub fn update_text(&mut self, text: String) {
        self.text = text;
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

    // 生成一个新条目
    fn new_price(raw_prices: &RawItem) -> PItem {
        return PItem {
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
            ..PItem::default()
        };
    }

    // 获取私有数据
    fn get_private_data(&self, symbol: &str) -> Option<&Private> {
        for item in &self.private_data {
            if item.symbol.to_lowercase() == symbol.to_lowercase() {
                return Some(item);
            }
        }
        return None;
    }

    // 设置关注
    fn set_marked(&mut self, index: usize, marked: bool) {
        if index >= self.data.len() {
            return;
        }

        self.data[index as usize].marked = marked;
        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
        self.save_private_data();
    }

    // 设置地板价格
    fn set_floor_price(&mut self, index: usize, price: f32) {
        if index >= self.data.len() {
            return;
        }

        self.data[index as usize].floor_price = price;
        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
        self.save_private_data();
    }

    // 添加条目
    fn add(&mut self, index: usize, raw_prices: &RawItem) {
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
    fn set(&mut self, index: usize, raw_prices: &RawItem) {
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

    // 条目不知列表中，则添加，在列表中则修改
    fn reset(&mut self, text: &str) {
        let raw_prices: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);
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

    // 清楚model
    fn clear(&mut self) {
        (self as &mut dyn QAbstractListModel).begin_reset_model();
        self.data = vec![];
        (self as &mut dyn QAbstractListModel).end_reset_model();
        self.count_changed();
    }

    // 插入行
    fn insert_rows(&mut self, row: usize, count: usize) -> bool {
        if count == 0 || row > self.data.len() {
            return false;
        }

        (self as &mut dyn QAbstractListModel)
            .begin_insert_rows(row as i32, (row + count - 1) as i32);
        for i in 0..count {
            self.data.insert(row + i, PItem::default());
        }
        (self as &mut dyn QAbstractListModel).end_insert_rows();
        self.count_changed();
        true
    }

    // 删除行
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

    // 交换行
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

    // 查找并与第一行交换
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
