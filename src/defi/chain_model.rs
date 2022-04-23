use cstr::cstr;
use qmetaobject::*;
use std::cmp::Ordering;

#[allow(unused_imports)]
use ::log::{debug, warn};

use crate::config::Config as conf;
use crate::defi::data::{ChainItem as Item, RawChainItem as RawItem};
use crate::defi::sort::{ChainSortKey as SortKey, SortDir};
use crate::utility;

/// 与qml交互的model对象
#[derive(QObject, Default)]
pub struct Model {
    base: qt_base_class!(trait QAbstractListModel),
    data: Vec<Item>,
    count: qt_property!(i32; READ row_count NOTIFY count_changed),
    count_changed: qt_signal!(),

    clear: qt_method!(fn(&mut self)),
    insert_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    remove_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    swap_row: qt_method!(fn(&mut self, from: usize, to: usize)),
    search_and_view_at_beginning: qt_method!(fn(&mut self, text: QString)),

    sort_by_key: qt_method!(fn(&mut self, key: u32)),
    toggle_sort_dir: qt_method!(fn(&mut self)),

    item_max_count: qt_property!(u32; NOTIFY item_max_count_changed),
    item_max_count_changed: qt_signal!(),

    // 缓存数据路径
    path: String,

    // 排序相关
    sort_key: u32,
    sort_dir: SortDir,

    // 更新数据相关
    text: String,
    text_changed: qt_signal!(),
    update_all: qt_method!(fn(&mut self)),
    pub url: String,

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
        engine.set_object_property("defi_chain_model".into(), model);
        qml_register_enum::<SortKey>(cstr!("DefiChainSortKey"), 1, 0, cstr!("DefiChainSortKey"));
    }

    // 设置默认值
    pub fn init_default(&mut self, config: &conf) {
        self.sort_key = SortKey::Index as u32;
        self.update_interval = config.defi_refresh_interval;
        self.update_now = false;
        self.item_max_count = config.defi_item_count;
        self.url = "https://api.llama.fi/chains".to_string();
    }

    // 设置缓存文件路径
    pub fn set_path(&mut self, filepath: &str) {
        self.path = filepath.to_string();
    }

    // 加载本地缓存数据
    pub fn load(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.path) {
            if text.is_empty() {
                return;
            }

            self.reset(&text);
            self.sort_by_key(self.sort_key);
        }
    }

    // 缓存数据到本地
    fn save(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.path, text) {
            warn!("write to {} error", &self.path);
        }
    }

    // 更新model
    fn update_all(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.clone();
        self.reset(&text);
        self.save(&text);
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
                } else if key == SortKey::TVL {
                    self.data
                        .sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(Ordering::Less));
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
    fn new(raw_item: &RawItem) -> Item {
        return Item {
            name: raw_item.name.clone().into(),
            symbol: raw_item.symbol.clone().unwrap_or("-".to_string()).into(),
            tvl: raw_item.tvl,
            ..Default::default()
        };
    }

    // 添加条目
    fn add(&mut self, index: usize, raw_item: &RawItem) {
        let end = self.data.len();
        (self as &mut dyn QAbstractListModel).begin_insert_rows(end as i32, end as i32);

        let mut item = Self::new(&raw_item);
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

        let mut item = Self::new(&raw_item);
        item.index = index as i32;
        self.data[index] = item;

        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
    }

    // 条目不知列表中，则添加，在列表中则修改
    fn reset(&mut self, text: &str) {
        let mut raw_item: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);

        raw_item.sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(Ordering::Less));

        for (i, item) in raw_item.iter().enumerate() {
            if i >= self.item_max_count as usize {
                break;
            }

            if self.data.len() <= i {
                self.add(i, &item);
            } else {
                self.set(i, &item);
            }
        }
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
            self.data.insert(row + i, Item::default());
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
    fn search_and_view_at_beginning(&mut self, text: QString) {
        if let Some(index) = self
            .data
            .iter()
            .position(|a| a.symbol.to_lower() == text.to_lower())
        {
            self.swap_row(0, index);
        }
    }
}
