use modeldata::*;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};
use platform_dirs::AppDirs;
use serde_derive::{Deserialize, Serialize};
use HandBookItem as Item;
use HandBookSubItem as SubItem;

#[derive(Serialize, Deserialize, Default, Debug)]
struct RawSubItem {
    is_sell: bool,
    total_price: f32,
    count: f32,
    time: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct RawItem {
    name: String,
    datas: Vec<RawSubItem>,
}

#[derive(QGadget, Clone, Default)]
pub struct HandBookItem {
    name: qt_property!(QString),
}

#[derive(QGadget, Clone, Default)]
pub struct HandBookSubItem {
    is_sell: qt_property!(bool),
    time: qt_property!(QString),
    total_price: qt_property!(f32),
    count: qt_property!(f32),
}

type SubModelVec = Vec<Box<SubModel>>;
modeldata_struct!(SubModel, SubItem, {}, {}, {});

modeldata_struct!(Model, Item, {
        path: String,
        sub_models: SubModelVec,
    }, {
    }, {
        save: fn(&mut self),
        add_item_qml: fn(&mut self, name: QString),
        set_item_qml: fn(&mut self, index: usize, name: QString),
        up_item_qml: fn(&mut self, index: usize),
        down_item_qml: fn(&mut self, index: usize),
        remove_item_qml: fn(&mut self, index: usize),

        sub_model_len: fn(&self, index: usize) -> u32,
        sub_model_item: fn(&mut self, index: usize, sub_index: usize) -> QVariant,

        add_sub_model_item_qml: fn(&mut self, index: usize, is_sell: bool, time: QString, total_price: f32, count: f32),

        remove_sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize),

        up_sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize),
        down_sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize),

        set_sub_model_item_qml: fn(&mut self, index: usize, sub_index: usize, is_sell: bool, time: QString, total_price: f32, count: f32),
    }
);

impl SubModel {
    fn add_item(&mut self, raw_items: &Vec<RawSubItem>) {
        for item in raw_items {
            let sub_item = SubItem {
                is_sell: item.is_sell,
                total_price: item.total_price,
                count: item.count,
                time: item.time.clone().into(),
            };
            self.append(sub_item);
        }
    }
}

impl Model {
    pub fn init(&mut self, app_dirs: &AppDirs) {
        self.path = app_dirs
            .data_dir
            .join("handbook.json")
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

    fn save(&mut self) {
        let mut raw_items = vec![];
        for (i, item) in self.items().iter().enumerate() {
            if i >= self.sub_models.len() {
                break;
            }

            let sub_model = &self.sub_models[i];
            let mut raw_sub_items = vec![];
            for sub_item in sub_model.items() {
                raw_sub_items.push(RawSubItem {
                    time: sub_item.time.to_string(),
                    is_sell: sub_item.is_sell,
                    total_price: sub_item.total_price,
                    count: sub_item.count,
                });
            }

            raw_items.push(RawItem {
                name: item.name.to_string(),
                datas: raw_sub_items,
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&raw_items) {
            if let Err(_) = std::fs::write(&self.path, text) {
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
        if index <= 0 {
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

    fn sub_model_len(&self, index: usize) -> u32 {
        if index >= self.sub_models.len() {
            return 0;
        }
        return self.sub_models[index].items_len() as u32;
    }

    fn sub_model_item(&mut self, index: usize, sub_index: usize) -> QVariant {
        if index >= self.sub_models.len() {
            return SubItem::default().to_qvariant();
        }

        return self.sub_models[index].item(sub_index);
    }

    fn add_sub_model_item(&mut self, index: usize, item: SubItem) {
        if index >= self.sub_models.len() {
            return;
        }
        self.sub_models[index].append(item);
    }

    fn add_sub_model_item_qml(
        &mut self,
        index: usize,
        is_sell: bool,
        time: QString,
        total_price: f32,
        count: f32,
    ) {
        let item = SubItem {
            time,
            is_sell,
            total_price,
            count,
        };
        self.add_sub_model_item(index, item);
    }

    fn remove_sub_model(&mut self, index: usize) {
        if index >= self.sub_models.len() {
            return;
        }

        self.sub_models.remove(index);
    }

    fn up_sub_model(&mut self, index: usize) {
        if index <= 0 {
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
        if index >= self.sub_models.len() || sub_index <= 0 {
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
        is_sell: bool,
        time: QString,
        total_price: f32,
        count: f32,
    ) {
        if index >= self.sub_models.len() || sub_index >= self.sub_models[index].items_len() {
            return;
        }
        let item = SubItem {
            is_sell,
            time,
            total_price,
            count,
        };
        self.sub_models[index].set(sub_index, item);
    }
}
