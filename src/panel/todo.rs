use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

use serde_derive::{Deserialize, Serialize};

/// 从json文件中解析出来的条目对象
#[derive(Serialize, Deserialize, Debug)]
struct RawItem {
    is_finished: bool,
    text: String,
}

/// 与qml交互的条目对象
#[derive(QGadget, Clone, Default)]
struct TItem {
    is_finished: qt_property!(bool),
    text: qt_property!(QString),
}

/// 与qml交互的model对象
#[derive(QObject, Default)]
pub struct Model {
    base: qt_base_class!(trait QAbstractListModel),
    data: Vec<TItem>,
    count: qt_property!(i32; READ row_count NOTIFY count_changed),
    count_changed: qt_signal!(),
    path: String, // 配置文件路径

    add: qt_method!(fn(&mut self, is_finished: bool, text: QString)),
    set: qt_method!(fn(&mut self, index: usize, is_finished: bool, text: QString)),
    save: qt_method!(fn(&mut self)),

    clear: qt_method!(fn(&mut self)),
    insert_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
    remove_rows: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
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
        engine.set_object_property("todo_model".into(), model);
    }

    pub fn set_path(&mut self, filepath: &str) {
        self.path = filepath.to_string();
    }

    pub fn load(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.path) {
            if let Ok(data) = serde_json::from_str::<Vec<RawItem>>(&text) {
                for i in &data {
                    self.data.push(TItem {
                        is_finished: i.is_finished,
                        text: i.text.clone().into(),
                    });
                }
            }
        }
    }

    fn save(&mut self) {
        let mut raw_item = vec![];
        for i in &self.data {
            raw_item.push(RawItem {
                is_finished: i.is_finished,
                text: i.text.to_string(),
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&raw_item) {
            if let Err(_) = std::fs::write(&self.path, text) {
                warn!("save {:?} failed", &self.path);
            }
        }
    }

    /// 添加条目
    fn add(&mut self, is_finished: bool, text: QString) {
        let end = self.data.len();
        (self as &mut dyn QAbstractListModel).begin_insert_rows(end as i32, end as i32);

        self.data.insert(end, TItem { is_finished, text });
        (self as &mut dyn QAbstractListModel).end_insert_rows();
        self.count_changed();
    }

    /// 修改条目
    fn set(&mut self, index: usize, is_finished: bool, text: QString) {
        if index >= self.data.len() {
            return;
        }

        self.data[index] = TItem { is_finished, text };
        let idx = (self as &mut dyn QAbstractListModel).row_index(index as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
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
            self.data.insert(row + i, TItem::default());
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
}
