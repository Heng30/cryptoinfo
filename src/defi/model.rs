use cstr::cstr;
use qmetaobject::*;
use std::cmp::Ordering;

#[allow(unused_imports)]
use ::log::{debug, warn};

use crate::config::Config as conf;
use crate::defi::data::{DefiItem, RawItem};
use crate::defi::sort::{SortDir, SortKey};
use crate::utility;

/// 与qml交互的model对象
#[derive(QObject, Default)]
pub struct Model {
    base: qt_base_class!(trait QAbstractListModel),
    data: Vec<DefiItem>,
    count: qt_property!(i32; READ row_count NOTIFY count_changed),
    count_changed: qt_signal!(),

    clear: qt_method!(fn(&mut self)),
    insert_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    remove_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    swap_row: qt_method!(fn(&mut self, from: usize, to: usize)),

    sort_by_key: qt_method!(fn(&mut self, key: u32)),
    toggle_sort_dir: qt_method!(fn(&mut self)),

    bull_percent: qt_property!(f32; NOTIFY bull_percent_changed), // 上涨占比
    bull_percent_changed: qt_signal!(),

    item_max_count: qt_property!(u32; NOTIFY item_max_count_changed),
    item_max_count_changed: qt_signal!(),

    // 缓存数据路径
    defi_path: String,

    // 排序相关
    sort_key: u32,
    sort_dir: SortDir,

    // 更新数据相关
    text: String,
    text_changed: qt_signal!(),
    update_all_defi: qt_method!(fn(&mut self)),
    pub defi_url: String,

    pub update_interval: qt_property!(u32; NOTIFY update_interval_changed), // 更新时间间隔
    update_interval_changed: qt_signal!(),

    pub update_now: qt_property!(bool; NOTIFY update_now_changed), // 马上更新
    update_now_changed: qt_signal!(),

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
        engine.set_object_property("defi_model".into(), model);
        qml_register_enum::<SortKey>(cstr!("DefiSortKey"), 1, 0, cstr!("DefiSortKey"));
    }

    // 设置默认值
    pub fn init_default(&mut self, config: &conf) {
        self.sort_key = SortKey::Index as u32;
        self.update_interval = config.defi_refresh_interval;
        self.update_now = false;
        self.item_max_count = config.defi_item_count;
        self.defi_url = "https://api.llama.fi/protocols".to_string();
    }

    // 设置缓存文件路径
    pub fn set_defi_path(&mut self, filepath: &str) {
        self.defi_path = filepath.to_string();
    }

    // 加载本地缓存数据
    pub fn load_defi(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.defi_path) {
            if text.is_empty() {
                return;
            }

            self.reset(&text);
            self.sort_by_key(self.sort_key);
        }
    }

    // 缓存数据到本地
    fn save_defi(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.defi_path, text) {
            warn!("write to {} error", &self.defi_path);
        }
    }

    // 更新model
    fn update_all_defi(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.clone();
        self.reset(&text);
        self.save_defi(&text);
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
                    self.data.sort_by(|a, b| {
                        a.symbol
                            .to_string()
                            .to_lowercase()
                            .cmp(&b.symbol.to_string().to_lowercase())
                    });
                } else if key == SortKey::Name {
                    self.data.sort_by(|a, b| {
                        a.name
                            .to_string()
                            .to_lowercase()
                            .cmp(&b.name.to_string().to_lowercase())
                    });
                } else if key == SortKey::Index {
                    self.data.sort_by(|a, b| a.index.cmp(&b.index));
                } else if key == SortKey::Per1H {
                    self.data.sort_by(|a, b| {
                        a.percent_change_1h
                            .partial_cmp(&b.percent_change_1h)
                            .unwrap_or(Ordering::Less)
                    });
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
                } else if key == SortKey::MarketCap {
                    self.data.sort_by(|a, b| {
                        a.market_cap_usd
                            .partial_cmp(&b.market_cap_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Staking {
                    self.data.sort_by(|a, b| {
                        a.staking.partial_cmp(&b.staking).unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::TVL {
                    self.data
                        .sort_by(|a, b| a.tvl.partial_cmp(&b.tvl).unwrap_or(Ordering::Less));
                } else {
                    return;
                }
            }
            _ => {
                if key == SortKey::Symbol {
                    self.data.sort_by(|a, b| {
                        b.symbol
                            .to_string()
                            .to_lowercase()
                            .cmp(&a.symbol.to_string().to_lowercase())
                    });
                } else if key == SortKey::Name {
                    self.data.sort_by(|a, b| {
                        b.name
                            .to_string()
                            .to_lowercase()
                            .cmp(&a.name.to_string().to_lowercase())
                    });
                } else if key == SortKey::Index {
                    self.data.sort_by(|a, b| b.index.cmp(&a.index));
                } else if key == SortKey::Per1H {
                    self.data.sort_by(|a, b| {
                        b.percent_change_1h
                            .partial_cmp(&a.percent_change_1h)
                            .unwrap_or(Ordering::Less)
                    });
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
                } else if key == SortKey::MarketCap {
                    self.data.sort_by(|a, b| {
                        b.market_cap_usd
                            .partial_cmp(&a.market_cap_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::TVL {
                    self.data
                        .sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(Ordering::Less));
                } else if key == SortKey::Staking {
                    self.data.sort_by(|a, b| {
                        b.staking.partial_cmp(&a.staking).unwrap_or(Ordering::Less)
                    });
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
    fn new_item(raw_item: &RawItem) -> DefiItem {
        return DefiItem {
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

    // 添加条目
    fn add(&mut self, index: usize, raw_item: &RawItem) {
        let end = self.data.len();
        (self as &mut dyn QAbstractListModel).begin_insert_rows(end as i32, end as i32);

        let mut item = Self::new_item(&raw_item);
        item.index = index as i32;

        self.data.insert(end, item);
        (self as &mut dyn QAbstractListModel).end_insert_rows();
        self.count_changed();
    }

    // 修改条目
    fn set(&mut self, index: usize, raw_item: &RawItem) {
        if index >= self.data.len() {
            return;
        }

        let mut item = Self::new_item(&raw_item);
        item.index = index as i32;
        self.data[index] = item;

        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
    }

    // 条目不知列表中，则添加，在列表中则修改
    fn reset(&mut self, text: &str) {
        let raw_item: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);
        let mut bull_count = 0;
        let mut bear_count = 0;

        for (i, item) in raw_item.iter().enumerate() {
            if i >= self.item_max_count as usize {
                break;
            }

            if item.change_1d.unwrap_or(0.0) > 0.0 {
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
            self.data.insert(row + i, DefiItem::default());
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
}
