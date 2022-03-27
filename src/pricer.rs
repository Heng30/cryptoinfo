use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;

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

#[derive(QGadget, Clone, Default)]
struct Pricer {
    index: qt_property!(i32),
    marked: qt_property!(bool),

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

#[derive(Debug, PartialEq)]
enum SortDir {
    UP,
    DOWN,
}

impl Default for SortDir {
    fn default() -> Self {
        return SortDir::UP;
    }
}

#[derive(QObject, Default)]
pub struct Model {
    base: qt_base_class!(trait QAbstractListModel),
    data: Vec<Pricer>,
    count: qt_property!(i32; READ row_count NOTIFY count_changed),
    count_changed: qt_signal!(),

    bull_percent: qt_property!(f32; NOTIFY bull_percent_changed), // 上涨占比
    bull_percent_changed: qt_signal!(),

    clear: qt_method!(fn(&mut self)),
    get_marked: qt_method!(fn(&self, index: usize) -> bool),
    set_marked: qt_method!(fn(&mut self, index: usize, marked: bool)),
    set_all_marked: qt_method!(fn(&mut self, marked: bool)),
    insert_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    remove_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    swap_row: qt_method!(fn(&mut self, from: usize, to: usize)),
    search_and_show_at_beginning: qt_method!(fn(&mut self, text: QString)),

    update_all_price: qt_method!(fn(&mut self, text: QString)),
    sort_by_key: qt_method!(fn(&mut self, key: QString)),
    toggle_sort_dir: qt_method!(fn(&mut self)),

    // 配置文件路径
    price_path: String,
    marked_path: String,
    markeds: Vec<String>, // 关注条目列表

    // 排序相关
    sort_key: String,
    sort_dir: SortDir,
}

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
    }

    pub fn set_marked_path(&mut self, filepath: &str) {
        self.marked_path = filepath.to_string();
    }

    pub fn init_marked(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.marked_path) {
            self.markeds = text.split(',').into_iter().map(|s| s.into()).collect();
        }
    }

    pub fn set_price_path(&mut self, filepath: &str) {
        self.price_path = filepath.to_string();
    }

    pub fn init_price(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.price_path) {
            if text.is_empty() {
                return;
            }

            self.reset(&text);
        }
    }

    fn update_all_price(&mut self, text: QString) {
        let text = text.to_string();
        self.clear();
        self.reset(&text);
        self.save_prices(&text);
        self.sort_by_key(self.sort_key.clone().into());
    }

    fn save_prices(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.price_path, text) {
            warn!("write to {} error", &self.price_path);
        }
    }

    fn save_marked(&mut self) {
        self.markeds.clear();
        for i in &self.data {
            if !i.marked {
                continue;
            }
            self.markeds.push(i.symbol.to_string());
        }

        let text = self.markeds.join(",");
        if let Err(_) = std::fs::write(&self.marked_path, text) {
            warn!("write to {} error", &self.marked_path);
        }
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

    fn toggle_sort_dir(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    fn sort_by_key(&mut self, key: QString) {
        if self.data.is_empty() {
            return;
        }

        let key = key.to_string().to_lowercase();
        match self.sort_dir {
            SortDir::UP => {
                if key == "symbol".to_string() {
                    self.data
                        .sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
                } else if key == "index".to_string() {
                    self.data.sort_by(|a, b| a.index.cmp(&b.index));
                } else if key == "24h%".to_string() {
                    self.data.sort_by(|a, b| {
                        a.percent_change_24h
                            .partial_cmp(&b.percent_change_24h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == "24h_volume".to_string() {
                    self.data.sort_by(|a, b| {
                        a.volume_24h_usd
                            .partial_cmp(&b.volume_24h_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key== "price".to_string() {
                    self.data.sort_by(|a, b| {
                        a.price_usd
                            .partial_cmp(&b.price_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key== "marked".to_string() {
                    self.data.sort_by(|a, b| a.index.cmp(&b.index));
                    self.data.sort_by(|a, b| a.marked.cmp(&b.marked));
                } else {
                    return;
                }
            }
            _ => {
                if key== "symbol".to_string() {
                    self.data
                        .sort_by(|a, b| b.symbol.to_string().cmp(&a.symbol.to_string()));
                } else if key== "index".to_string() {
                    self.data.sort_by(|a, b| b.index.cmp(&a.index));
                } else if key== "24h%".to_string() {
                    self.data.sort_by(|a, b| {
                        b.percent_change_24h
                            .partial_cmp(&a.percent_change_24h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == "24h_volume".to_string() {
                    self.data.sort_by(|a, b| {
                        b.volume_24h_usd
                            .partial_cmp(&a.volume_24h_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key== "price".to_string() {
                    self.data.sort_by(|a, b| {
                        b.price_usd
                            .partial_cmp(&a.price_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key== "marked".to_string() {
                    self.data.sort_by(|a, b| a.index.cmp(&b.index));
                    self.data.sort_by(|a, b| b.marked.cmp(&a.marked));
                } else {
                    return;
                }
            }
        }

        self.sort_key = key.clone();

        let end = self.data.len();
        let idx1 = (self as &mut dyn QAbstractListModel).row_index(0);
        let idx2 = (self as &mut dyn QAbstractListModel).row_index((end - 1) as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx1, idx2);
    }

    fn add(&mut self, index: i32, raw_prices: &RawPricer) {
        let end = self.data.len();
        (self as &mut dyn QAbstractListModel).begin_insert_rows(end as i32, end as i32);

        self.data.insert(
            end,
            Pricer {
                index,
                marked: self.markeds.contains(&raw_prices.symbol),

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
            },
        );
        (self as &mut dyn QAbstractListModel).end_insert_rows();
        self.count_changed();
    }

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

            self.add(1 + i as i32, &item);
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

    fn get_marked(&self, index: usize) -> bool {
        if index >= self.data.len() {
            return false;
        }

        return (&self.data[index]).marked;
    }

    fn set_marked(&mut self, index: usize, marked: bool) {
        if index >= self.data.len() {
            return;
        }

        self.data[index as usize].marked = marked;
        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
        self.save_marked();
    }

    fn set_all_marked(&mut self, marked: bool) {
        for i in &mut self.data {
            i.marked = marked;
        }

        let idx1 = (self as &mut dyn QAbstractListModel).row_index(0);
        let end = self.data.len() as i32;
        let idx2 = (self as &mut dyn QAbstractListModel).row_index(end - 1);
        (self as &mut dyn QAbstractListModel).data_changed(idx1, idx2);
        self.save_marked();
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
