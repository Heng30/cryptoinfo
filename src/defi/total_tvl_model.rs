use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

use crate::config::Config as conf;
use crate::defi::data::{TotalTVLItem as Item, RawTotalTVLItem as RawItem};
use crate::utility;

/// 与qml交互的model对象
#[derive(QObject, Default)]
pub struct Model {
    base: qt_base_class!(trait QAbstractListModel),
    data: Vec<Item>,
    count: qt_property!(i32; READ row_count NOTIFY count_changed),
    count_changed: qt_signal!(),

    clear: qt_method!(fn(&mut self)),
    get_item: qt_method!(fn(&self, index: u32) -> QVariant),

    min_tvl: qt_property!(u64; NOTIFY min_tvl_changed),
    min_tvl_changed: qt_signal!(),

    max_tvl: qt_property!(u64; NOTIFY max_tvl_changed),
    max_tvl_changed: qt_signal!(),

    // 缓存数据路径
    path: String,
    pub url: String,

    // 更新数据相关
    text: String,
    text_changed: qt_signal!(),
    update_all: qt_method!(fn(&mut self)),
    updated: qt_signal!(count: u32),

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
        engine.set_object_property("defi_total_tvl_model".into(), model);
    }

    // 设置默认值
    pub fn init_default(&mut self, config: &conf) {
        self.update_interval = config.defi_refresh_interval;
        self.update_now = false;
        self.url = "https://api.llama.fi/charts".to_string();
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
            self.updated(self.data.len() as u32);
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
        self.update_time = utility::Utility::default().local_time_now(QString::from("%H:%M:%S"));
        self.update_time_changed();
        self.updated(self.data.len() as u32);
    }

    // 更新数据
    pub fn update_text(&mut self, text: String) {
        self.text = text;
        self.text_changed();
    }

    // 生成一个新条目
    fn new(raw_item: &RawItem) -> Item {
        return Item {
            second: raw_item.date.parse().unwrap_or(0),
            tvl: raw_item.tvl as u64,
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
        let raw_item: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);
        let mut min_tvl = u64::max_value();
        let mut max_tvl = 0;

        for (i, item) in raw_item.iter().enumerate() {
            min_tvl = u64::min(item.tvl as u64, min_tvl);
            max_tvl = u64::max(item.tvl as u64, max_tvl);
            if self.data.len() <= i {
                self.add(i, &item);
            } else {
                self.set(i, &item);
            }
        }

        if max_tvl > min_tvl {
            self.max_tvl = max_tvl;
            self.min_tvl = min_tvl;
            self.max_tvl_changed();
            self.min_tvl_changed();
        }
    }

    // 清楚model
    fn clear(&mut self) {
        (self as &mut dyn QAbstractListModel).begin_reset_model();
        self.data = vec![];
        (self as &mut dyn QAbstractListModel).end_reset_model();
        self.count_changed();
    }

    fn get_item(&self, index: u32) -> QVariant {
        return self
            .data
            .get(index as usize)
            .map(|x| x.to_qvariant())
            .unwrap_or_default();
    }
}
