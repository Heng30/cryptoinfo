use crate::qobjmgr::{qobj, NodeType as QNodeType};
use modeldata::*;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};
use platform_dirs::AppDirs;
use serde_derive::{Deserialize, Serialize};
use FundBookItem as Item;

#[derive(Serialize, Deserialize, Default, Debug)]
struct RawItem {
    time: String,
    crypto: f32,
    stock: f32,
    saving: f32,
    other: f32,
}

#[derive(QGadget, Clone, Default)]
pub struct FundBookItem {
    time: qt_property!(QString),
    crypto: qt_property!(f32),
    stock: qt_property!(f32),
    saving: qt_property!(f32),
    other: qt_property!(f32),
}

modeldata_struct!(Model, Item, members: {
        path: String,
    }, members_qt: {
    }, signals_qt: {
    }, methods_qt: {
        save_qml: fn(&mut self),
        add_item_qml: fn(&mut self, time: QString, crypto: f32, stock: f32, saving: f32, other: f32),
        set_item_qml: fn(&mut self, index: usize, time: QString, crypto: f32, stock: f32, saving: f32, other: f32),
        up_item_qml: fn(&mut self, index: usize),
        down_item_qml: fn(&mut self, index: usize),
        remove_item_qml: fn(&mut self, index: usize),
        up_join_item_qml: fn(&mut self, index: usize) -> bool,
        stats_qml: fn(&mut self) -> QString ,
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        self.path = app_dirs
            .data_dir
            .join("fundbook.json")
            .to_str()
            .unwrap()
            .to_string();
        self.load();
    }

    pub fn load(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.path) {
            if let Ok(raw_items) = serde_json::from_str::<Vec<RawItem>>(&text) {
                for item in &raw_items {
                    self.add_item(item);
                }
            }
        }
    }

    fn save_qml(&mut self) {
        let mut raw_items = vec![];
        for item in self.items().iter() {
            raw_items.push(RawItem {
                time: item.time.clone().into(),
                crypto: item.crypto,
                stock: item.stock,
                saving: item.saving,
                other: item.other,
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&raw_items) {
            if let Err(_) = std::fs::write(&self.path, &text) {
                warn!("save {:?} failed", &self.path);
            }
        }
    }

    fn new_item(&self, time: &str, crypto: f32, stock: f32, saving: f32, other: f32) -> Item {
        return Item {
            time: time.to_string().into(),
            crypto,
            stock,
            saving,
            other,
        };
    }

    fn add_item(&mut self, raw_item: &RawItem) {
        let item = self.new_item(
            &raw_item.time,
            raw_item.crypto,
            raw_item.stock,
            raw_item.saving,
            raw_item.other,
        );
        self.append(item);
    }

    fn add_item_qml(&mut self, time: QString, crypto: f32, stock: f32, saving: f32, other: f32) {
        let item = self.new_item(&time.to_string(), crypto, stock, saving, other);
        self.append(item);
    }

    fn set_item_qml(
        &mut self,
        index: usize,
        time: QString,
        crypto: f32,
        stock: f32,
        saving: f32,
        other: f32,
    ) {
        let item = self.new_item(&time.to_string(), crypto, stock, saving, other);
        self.set(index, item);
    }

    fn remove_item_qml(&mut self, index: usize) {
        self.remove_rows(index, 1);
    }

    fn up_item_qml(&mut self, index: usize) {
        if index <= 0 {
            return;
        }
        self.swap_row(index - 1, index);
    }

    fn down_item_qml(&mut self, index: usize) {
        if index >= self.items_len() - 1 {
            return;
        }
        self.swap_row(index, index + 1);
    }

    fn up_join_item_qml(&mut self, index: usize) -> bool {
        if index <= 0 || index >= self.items_len() {
            return false;
        }

        let item_1 = &self.items()[index - 1];
        let item_2 = &self.items()[index];

        let item = Item {
            time: item_1.time.to_string().into(),
            crypto: item_1.crypto + item_2.crypto,
            stock: item_1.stock + item_2.stock,
            saving: item_1.saving + item_2.saving,
            other: item_1.other + item_2.other,
        };

        self.set(index - 1, item);
        self.remove_rows(index, 1);
        return true;
    }

    fn stats_qml(&mut self) -> QString {
        let mut crypto = 0.0f32;
        let mut stock = 0.0f32;
        let mut saving = 0.0f32;
        let mut other = 0.0f32;
        for item in self.items() {
            crypto = crypto + item.crypto;
            stock = stock + item.stock;
            saving = saving + item.saving;
            other = other + item.other;

        }

        let total = crypto + stock + saving + other;

        return format!("{}, {},{},{},{}", total, crypto, stock, saving, other).into();
    }
}
