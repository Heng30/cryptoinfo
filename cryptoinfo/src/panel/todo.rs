use crate::qobjmgr::{qobj, NodeType as QNodeType};
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};
use TodoItem as Item;

#[allow(unused_imports)]
use ::log::{debug, warn};

/// 从json文件中解析出来的条目对象
#[derive(Serialize, Deserialize, Debug)]
struct RawItem {
    is_finished: bool,
    text: String,
}

/// 与qml交互的条目对象
#[derive(QGadget, Clone, Default)]
pub struct TodoItem {
    is_finished: qt_property!(bool),
    text: qt_property!(QString),
}

modeldata_struct!(Model, Item, {
        path: String,
    }, {
    }, {
        save: fn(&mut self),
        add_item: fn(&mut self, is_finished: bool, text: QString),
        set_item: fn(&mut self, index: usize, is_finished: bool, text: QString),
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        let file = app_dirs.data_dir.join("todo.dat");
        self.path = file.to_str().unwrap().to_string();
        self.load();
    }

    // 加载缓存
    fn load(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.path) {
            if let Ok(data) = serde_json::from_str::<Vec<RawItem>>(&text) {
                for i in &data {
                    self.append(Item {
                        is_finished: i.is_finished,
                        text: i.text.clone().into(),
                    });
                }
            }
        }
    }

    // 缓存到本地
    fn save(&mut self) {
        let mut raw_item = vec![];
        for item in self.items() {
            raw_item.push(RawItem {
                is_finished: item.is_finished,
                text: item.text.to_string(),
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&raw_item) {
            if let Err(_) = std::fs::write(&self.path, text) {
                warn!("save {:?} failed", &self.path);
            }
        }
    }

    /// 添加条目
    fn add_item(&mut self, is_finished: bool, text: QString) {
        self.append(Item { is_finished, text });
    }

    /// 修改条目
    pub fn set_item(&mut self, index: usize, is_finished: bool, text: QString) {
        self.set(index, Item { is_finished, text });
    }
}
