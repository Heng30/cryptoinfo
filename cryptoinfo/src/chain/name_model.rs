use super::data::ChainNamesItem as Item;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
#[allow(unused_imports)]
use ::log::{debug, warn};
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;

modeldata_struct!(Model, Item, members: {
        path: String,
    }, members_qt: {
    }, signals_qt: {
    }, methods_qt: {
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.path = app_dirs
            .data_dir
            .join("chain-names.json")
            .to_str()
            .unwrap()
            .to_string();
        self.load();
    }

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
