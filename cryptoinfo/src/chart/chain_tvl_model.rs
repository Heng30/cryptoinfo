use super::data::{ChainTVLItem as Item, RawChainTVLItem as RawItem};
use crate::httpclient;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use std::collections::HashMap;
use tokio::{self, time};

type NameMap = HashMap<String, String>;

modeldata_struct!(Model, Item, members: {
        dir: String,
        name_map: NameMap,
        is_update_cache: bool,
    }, members_qt: {
        min_tvl: [u64; min_tvl_changed],
        max_tvl: [u64; max_tvl_changed],
        name: [QString; name_changed],
        text: [QString; text_changed],
        update_now: [bool; pdate_now_changed], // 马上更新
        update_time: [QString; update_time_changed], // 数据更新时间
    }, signals_qt: {
    }, methods_qt: {
        update_text_qml: fn(&mut self, name: QString),
        likely_item_qml: fn(&mut self, second: u64) -> QVariant,
        update_all_qml: fn(&mut self),
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.update_now = false;
        self.dir = app_dirs
            .data_dir
            .join("chain-tvl")
            .to_str()
            .unwrap()
            .to_string();

        update_defi_chain_tvl(QBox::new(self));
    }

    fn gen_path(&self, name: String) -> String {
        return self.dir.clone() + "/" + &name + ".json";
    }

    pub fn gen_url(&self) -> String {
        if self.name.is_empty() {
            return String::default();
        }

        // 总锁仓量
        if self.name == "Chains".to_string().into() {
            return "https://api.llama.fi/charts".to_string();
        }

        return "https://api.llama.fi/charts/".to_string() + self.name.to_string().as_ref();
    }

    fn update_text_qml(&mut self, name: QString) {
        if self.name == name {
            self.updated();
            return;
        }

        self.name = name;
        if self.name_map.contains_key(&self.name.to_string()) {
            self.text = self
                .name_map
                .get(&self.name.to_string())
                .unwrap()
                .clone()
                .into();
            self.text_changed();
        } else {
            let file = self.gen_path(self.name.to_string());

            if let Ok(text) = std::fs::read_to_string(&file) {
                if !text.is_empty() {
                    self.text = text.into();
                    self.text_changed();
                } else {
                    self.update_now = true;
                }
            } else {
                self.update_now = true;
            }
        }
    }

    // 缓存数据到本地
    fn save(&self, text: &str) {
        let file = self.gen_path(self.name.to_string());
        if let Err(_) = std::fs::write(&file, text) {
            warn!("write to {} error", &file);
        }
    }

    // 更新model
    fn update_all_qml(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.to_string();
        if self.is_update_cache {
            self.save(&text);
            self.is_update_cache = false;
        }

        if !self.name_map.contains_key(&self.name.to_string()) {
            self.name_map.insert(self.name.to_string(), text.clone());
        }

        self.reset(&text);
        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
        self.updated();
    }

    // 更新数据
    pub fn update_text(&mut self, name: String, text: String) {
        self.name = name.into();
        self.text = text.into();
        self.is_update_cache = true;
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
    fn add_item(&mut self, index: usize, raw_item: &RawItem) {
        let mut item = Self::new(&raw_item);
        item.index = index as i32;
        self.append(item);
    }

    fn reset(&mut self, text: &str) {
        let raw_item: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);
        let mut min_tvl = u64::max_value();
        let mut max_tvl = 0;

        if raw_item.is_empty() {
            return;
        }
        self.clear();

        for (i, item) in raw_item.iter().enumerate() {
            min_tvl = u64::min(item.tvl as u64, min_tvl);
            max_tvl = u64::max(item.tvl as u64, max_tvl);
            self.add_item(i, &item);
        }

        if max_tvl > min_tvl {
            self.max_tvl = max_tvl;
            self.min_tvl = min_tvl;
            self.max_tvl_changed();
            self.min_tvl_changed();
        }
    }

    fn likely_item_qml(&mut self, second: u64) -> QVariant {
        if self.items_is_empty() {
            return Item::default().to_qvariant();
        }

        let mut s = 0_usize;
        let mut e = self.items_len() as usize;
        let mut m = s / 2 + e / 2 as usize;
        while s < e {
            let item = &self.items()[m];
            if item.second == second {
                return item.to_qvariant();
            } else if item.second < second {
                s = m + 1;
            } else {
                e = m - 1;
            }
            m = s / 2 + e / 2 as usize;
        }

        if s >= self.items_len() {
            return self.items()[self.items_len()].to_qvariant();
        }

        return self.items()[s].to_qvariant();
    }
}

pub fn update_defi_chain_tvl(model: QBox<Model>) {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(1));

        loop {
            let url = model.borrow().gen_url();
            let name = model.borrow().name.to_string();

            if model.borrow().update_now && !url.is_empty() {
                if let Ok(res) = httpclient::http_get(&url).await {
                    model.borrow_mut().update_text(name, res);
                }
                model.borrow_mut().update_now = false;
            }
            interval.tick().await;
        }
    });
}
