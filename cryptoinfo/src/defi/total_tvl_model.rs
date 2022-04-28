use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

use crate::config::Config;
use super::data::{RawTotalTVLItem as RawItem, TotalTVLItem as Item};
use crate::utility;

modeldata_struct!(Model, Item, {
        path: String,
        url: String,
    }, {
        min_tvl: [u64; min_tvl_changed],
        max_tvl: [u64; max_tvl_changed],
        text: [QString; text_changed],
        update_interval: [u32; update_interval_changed], // 更新时间间隔
        update_now: [bool; pdate_now_changed], // 马上更新
        update_time: [QString; update_time_changed], // 数据更新时间
    }, {
        update_all: fn(&mut self),
        likely_item: fn(&mut self, second: u64) -> QVariant,
    }
);

impl Model {
    pub fn init(&mut self, config: &Config, app_dirs: &AppDirs) {
        self.update_interval = config.defi_refresh_interval;
        self.update_now = false;
        self.url = "https://api.llama.fi/charts".to_string();

        self.path = app_dirs
            .data_dir
            .join("defi-total-tvl.json")
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

            self.reset(&text);
            self.updated();
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

        let text = self.text.to_string();
        self.reset(&text);
        self.save(&text);
        self.update_time = utility::Utility::default().local_time_now(QString::from("%H:%M:%S"));
        self.update_time_changed();
        self.updated();
    }

    // 更新数据
    pub fn update_text(&mut self, text: String) {
        self.text = text.into();
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

    fn likely_item(&mut self, second: u64) -> QVariant {
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
