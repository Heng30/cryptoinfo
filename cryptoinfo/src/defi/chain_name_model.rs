use crate::qobjmgr::{qobj, NodeType as QNodeType};
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;

use super::data::ChainNamesItem as Item;
#[allow(unused_imports)]
use ::log::{debug, warn};

modeldata_struct!(Model, Item, {
        path: String,
    }, {
    }, {
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        self.path = app_dirs
            .data_dir
            .join("defi-chain-names.json")
            .to_str()
            .unwrap()
            .to_string();
        self.load();
    }

    // 加载本地缓存数据
    fn load(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.path) {
            if text.is_empty() {
                return;
            }

            let names: Vec<String> = serde_json::from_str(&text).unwrap_or(vec![]);
            for name in &names {
                self.append(Item {
                    name: name.clone().into(),
                });
            }
        }
    }
}
