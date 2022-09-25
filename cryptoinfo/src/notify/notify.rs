use super::data::{NotifyItem as Item, RawNotifyItem as RawItem};
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::utility::Utility;
use ::log::{debug, warn};
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::prelude::*;
use std::fs;
use std::path::Path;

modeldata_struct!(Model, Item, members: {
        filepath: String,
    }, members_qt: {
        has_unread: [bool; has_unread_changed],
    }, signals_qt: {
    }, methods_qt: {
        save_qml: fn(&mut self),
        add_item_qml: fn(&mut self, timestamp: i64,  module: QString, level: i32, content: QString),
        remove_item_qml: fn(&mut self, index: i32),
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.filepath = app_dirs
            .data_dir
            .join("tmp/notify.json")
            .to_str()
            .unwrap()
            .to_string();
        self.load();
    }

    fn load(&mut self) {
        let filepath = Path::new(&self.filepath);
        if !filepath.exists() {
            return;
        }

        match fs::read_to_string(&filepath) {
            Err(e) => debug!("{:?}", e),
            Ok(text) => {
                if let Ok(raw_items) = serde_json::from_str::<Vec<RawItem>>(&text) {
                    for item in raw_items.into_iter() {
                        self.append(Item {
                            timestamp: item.timestamp.into(),
                            level: item.level,
                            module: item.module.into(),
                            content: item.content.into(),
                        })
                    }
                    self.has_unread = !self.items_is_empty();
                    self.has_unread_changed();
                }
            }
        }
    }

    fn save_qml(&mut self) {
        let mut raw_items = vec![];
        for item in self.items().iter() {
            raw_items.push(RawItem {
                timestamp: item.timestamp.to_string(),
                level: item.level,
                module: item.module.to_string(),
                content: item.content.to_string(),
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&raw_items) {
            if let Err(e) = fs::write(&self.filepath, text) {
                warn!("save {:?} failed. error: {:?}", &self.filepath, e);
            }
        }

        self.has_unread = !self.items_is_empty();
        self.has_unread_changed();
    }

    fn add_item_qml(&mut self, timestamp: i64, module: QString, level: i32, content: QString) {
        if content.is_empty() {
            return;
        }

        if self.items_len() > 100 {
            self.remove_rows(0, 1);
        }

        self.append(Item {
            timestamp: Utility::utc_seconds_to_local_string(timestamp, "%m-%d %H:%M:%S").into(),
            level,
            module,
            content,
        });
        self.save_qml();
    }

    fn remove_item_qml(&mut self, index: i32) {
        if index < 0 || index as usize >= self.items_len() {
            return;
        }
        let index = index as usize;
        self.remove_rows(index, 1);
        self.save_qml();
    }
}
