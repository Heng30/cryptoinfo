use crate::qobjmgr::{qobj, NodeType as QNodeType};
use modeldata::*;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};
use platform_dirs::AppDirs;
use serde_derive::{Deserialize, Serialize};
use BookMarkItem as Item;
use BookMarkSubItem as SubItem;

#[derive(Serialize, Deserialize, Default, Debug)]
struct RawSubItem {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct RawItem {
    name: String,
    datas: Vec<RawSubItem>,
}

#[derive(QGadget, Clone, Default)]
pub struct BookMarkItem {
    name: qt_property!(QString),
}

#[derive(QGadget, Clone, Default)]
pub struct BookMarkSubItem {
    name: qt_property!(QString),
    url: qt_property!(QString),
}

type SubModelVec = Vec<Box<SubModel>>;
modeldata_struct!(SubModel, SubItem, members: {}, members_qt: {}, signals_qt: {}, methods_qt: {});

modeldata_struct!(Model, Item, members: {
        path: String,
        sub_models: SubModelVec,
    }, members_qt: {
    }, signals_qt: {
    }, methods_qt: {
        save_qml: fn(&mut self),
        add_item_qml: fn(&mut self, name: QString),
        set_item_qml: fn(&mut self, index: usize, name: QString),
        up_item_qml: fn(&mut self, index: usize),
        down_item_qml: fn(&mut self, index: usize),
        remove_item_qml: fn(&mut self, index: usize),

        sub_model_len_qml: fn(&self, index: usize) -> u32,
        sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize) -> QVariant,

        add_sub_model_item_qml: fn(&mut self, index: usize, name: QString, url: QString),

        remove_sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize),

        up_sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize),
        down_sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize),

        set_sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize, name: QString, url: QString),
    }
);

impl SubModel {
    fn add_item(&mut self, raw_items: &Vec<RawSubItem>) {
        for item in raw_items {
            let sub_item = SubItem {
                name: item.name.clone().into(),
                url: item.url.clone().into(),
            };
            self.append(sub_item);
        }
    }
}

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.path = app_dirs
            .data_dir
            .join("bookmark.json")
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
        for (i, item) in self.items().iter().enumerate() {
            if i >= self.sub_models.len() {
                break;
            }

            let sub_model = &self.sub_models[i];
            let mut raw_sub_items = vec![];
            for sub_item in sub_model.items() {
                raw_sub_items.push(RawSubItem {
                    name: sub_item.name.to_string(),
                    url: sub_item.url.to_string(),
                });
            }

            raw_items.push(RawItem {
                name: item.name.to_string(),
                datas: raw_sub_items,
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&raw_items) {
            if std::fs::write(&self.path, text).is_err() {
                warn!("save {:?} failed", &self.path);
            }
        }
    }

    fn new_item(&self, name: &str) -> Item {
        return Item {
            name: name.to_string().into(),
        };
    }

    fn add_item(&mut self, raw_item: &RawItem) {
        let item = self.new_item(&raw_item.name);
        self.append(item);
        let mut sub_model = Box::new(SubModel::default());
        sub_model.internal_init();
        sub_model.add_item(&raw_item.datas);
        self.sub_models.push(sub_model);
    }

    fn add_item_qml(&mut self, name: QString) {
        let item = self.new_item(&name.to_string());
        self.append(item);
        let mut sub_model = Box::new(SubModel::default());
        sub_model.internal_init();
        self.sub_models.push(sub_model);
    }

    fn set_item_qml(&mut self, index: usize, name: QString) {
        let item = self.new_item(&name.to_string());
        self.set(index, item);
    }

    fn remove_item_qml(&mut self, index: usize) {
        self.remove_rows(index, 1);
        self.remove_sub_model(index);
    }

    fn up_item_qml(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        self.swap_row(index - 1, index);
        self.up_sub_model(index);
    }

    fn down_item_qml(&mut self, index: usize) {
        if index >= self.items_len() - 1 {
            return;
        }
        self.swap_row(index, index + 1);
        self.down_sub_model(index);
    }

    fn sub_model_len_qml(&self, index: usize) -> u32 {
        if index >= self.sub_models.len() {
            return 0;
        }
        return self.sub_models[index].items_len() as u32;
    }

    fn sub_model_item_qml(&mut self, index: usize, sub_index: usize) -> QVariant {
        if index >= self.sub_models.len() {
            return SubItem::default().to_qvariant();
        }

        return self.sub_models[index].item_qml(sub_index);
    }

    fn add_sub_model_item(&mut self, index: usize, item: SubItem) {
        if index >= self.sub_models.len() {
            return;
        }
        self.sub_models[index].append(item);
    }

    fn add_sub_model_item_qml(&mut self, index: usize, name: QString, url: QString) {
        let item = SubItem { name, url };
        self.add_sub_model_item(index, item);
    }

    fn remove_sub_model(&mut self, index: usize) {
        if index >= self.sub_models.len() {
            return;
        }

        self.sub_models.remove(index);
    }

    fn up_sub_model(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        self.sub_models.swap(index - 1, index);
    }

    fn down_sub_model(&mut self, index: usize) {
        if index >= self.sub_models.len() {
            return;
        }
        self.sub_models.swap(index, index + 1);
    }

    fn remove_sub_model_item_qml(&mut self, index: usize, sub_index: usize) {
        if index >= self.sub_models.len() {
            return;
        }

        self.sub_models[index].remove_rows(sub_index, 1);
    }

    fn up_sub_model_item_qml(&mut self, index: usize, sub_index: usize) {
        if index >= self.sub_models.len() || sub_index == 0 {
            return;
        }

        self.sub_models[index].swap_row(sub_index - 1, sub_index);
    }

    fn down_sub_model_item_qml(&mut self, index: usize, sub_index: usize) {
        if index >= self.sub_models.len() || sub_index >= self.sub_models[index].items_len() {
            return;
        }

        self.sub_models[index].swap_row(sub_index, sub_index + 1);
    }

    fn set_sub_model_item_qml(
        &mut self,
        index: usize,
        sub_index: usize,
        name: QString,
        url: QString,
    ) {
        if index >= self.sub_models.len() || sub_index >= self.sub_models[index].items_len() {
            return;
        }
        let item = SubItem { name, url };
        self.sub_models[index].set(sub_index, item);
    }
}
