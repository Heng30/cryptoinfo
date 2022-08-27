use super::data::{PriceItem as Item, Private, RawItem};
use super::sort::{SortDir, SortKey};
use crate::config::{Config, PanelType};
use crate::httpclient;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::utility::Utility;
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use std::cmp::Ordering;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering as AOrdering};
use std::sync::Mutex;

type PrivateVec = Vec<Private>;
type ItemVec = Mutex<Option<Vec<Item>>>;
type MString = std::sync::Mutex<String>;

modeldata_struct!(Model, Item, members: {
        private_path: String,
        price_path: String,
        private: PrivateVec,
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: MString,
        update_now: AtomicBool,
        update_interval: AtomicU32,
    }, members_qt:{
        bull_percent: [f32; bull_percent_changed],
        item_max_count: [u32; item_max_count_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
    },
    methods_qt: {
        set_url_qml: fn(&mut self, limit: u32),
        set_marked_qml: fn(&mut self, index: usize, marked: bool),
        set_floor_price_qml: fn(&mut self, index: usize, price: f32),
        search_and_view_at_beginning_qml: fn(&mut self, text: QString),
        refresh_qml: fn(&mut self),
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
        set_update_interval_qml: fn(&mut self, interval: u32),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow().url.lock().unwrap().clone();
    }

    fn update_interval(&self) -> usize {
        return self.borrow().update_interval.load(AOrdering::SeqCst) as usize;
    }

    fn update_now(&self) -> bool {
        return self.borrow().update_now.load(AOrdering::SeqCst);
    }

    fn disable_update_now(&self) {
        self.borrow().update_now.store(false, AOrdering::SeqCst);
    }

    fn parse_body(&mut self, text: &str) {
        self.borrow().save(text);

        // TODO: is unsafe across threads
        if Model::can_updated() {
            self.borrow_mut().cache_items(text);
        }
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(cstr!("PriceSortKey"), 1, 0, cstr!("PriceSortKey"));

        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        let config = qobj::<Config>(QNodeType::Config);
        self.sort_key = SortKey::Marked as u32;
        self.update_interval = AtomicU32::new(config.price_refresh_interval);
        self.set_url_qml(config.price_item_count);

        let file = app_dirs.data_dir.join("private.json");
        self.private_path = file.to_str().unwrap().to_string();

        let file = app_dirs.data_dir.join("tmp/price.json");
        self.price_path = file.to_str().unwrap().to_string();

        self.load_private();
        self.async_update_model();
    }

    fn can_updated() -> bool {
        let conf = qobj::<Config>(QNodeType::Config);
        if conf.unrefresh_when_not_focus && conf.panel_type != PanelType::Price as u32 {
            return false;
        }
        return true;
    }

    fn load_private(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.private_path) {
            match serde_json::from_str::<PrivateVec>(&text) {
                Ok(data) => self.private = data,
                Err(e) => debug!("{:?}", e),
            }
        }
    }

    fn save(&self, text: &str) {
        if let Err(e) = std::fs::write(&self.price_path, text) {
            warn!("save {:?} failed. error: {:?}", &self.price_path, e);
        }
    }

    fn save_private(&mut self) {
        self.private.clear();
        for i in &self.inner_model.data {
            if !i.marked && i.floor_price < 0.00001 {
                continue;
            }
            self.private.push(Private {
                symbol: i.symbol.to_string(),
                marked: i.marked,
                floor_price: i.floor_price,
            });
        }

        if let Ok(text) = serde_json::to_string_pretty(&self.private) {
            if let Err(_) = std::fs::write(&self.private_path, text) {
                warn!("save {:?} failed", &self.private_path);
            }
        }
    }

    fn update_model(&mut self, _text: String) {
        if !Model::can_updated() {
            return;
        }

        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }

        let tmp_items = tmp_items.unwrap();
        if tmp_items.len() < self.items_len() {
            self.remove_rows(tmp_items.len(), self.items_len() - tmp_items.len());
        }

        for (i, item) in tmp_items.into_iter().enumerate() {
            if self.items_len() > i {
                self.set(i, item);
            } else {
                self.append(item);
            }
        }

        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.sort_by_key_qml(self.sort_key);
        self.update_time_changed();
    }

    pub fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 1, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<Vec<RawItem>>(text) {
            Ok(raw_prices) => {
                let mut bull_count = 0;
                let mut bear_count = 0;
                let mut v = vec![];

                for (i, item) in raw_prices.into_iter().enumerate() {
                    if i >= self.item_max_count as usize {
                        break;
                    }

                    if item.percent_change_24h.parse().unwrap_or(0.0) > 0.0 {
                        bull_count += 1;
                    } else {
                        bear_count += 1;
                    }

                    let mut it = Self::new_price(item);
                    it.index = i as i32;
                    if let Some(pdata) = self.get_private(&it.symbol.to_string()) {
                        it.marked = pdata.marked;
                        it.floor_price = pdata.floor_price;
                    }
                    v.push(it);
                }
                *self.tmp_items.lock().unwrap() = Some(v);

                if bear_count + bull_count > 0 {
                    self.bull_percent = bull_count as f32 / (bull_count + bear_count) as f32;
                    self.bull_percent_changed();
                }
            }
            Err(e) => debug!("{:?}", e),
        }
    }

    fn sort_by_key_qml(&mut self, key: u32) {
        if self.items_is_empty() {
            return;
        }

        let key: SortKey = key.into();
        if key == SortKey::Symbol {
            self.items_mut()
                .sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
        } else if key == SortKey::Index {
            self.items_mut().sort_by(|a, b| a.index.cmp(&b.index));
        } else if key == SortKey::Per24H {
            self.items_mut().sort_by(|a, b| {
                a.percent_change_24h
                    .partial_cmp(&b.percent_change_24h)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Per7D {
            self.items_mut().sort_by(|a, b| {
                a.percent_change_7d
                    .partial_cmp(&b.percent_change_7d)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Volume24H {
            self.items_mut().sort_by(|a, b| {
                a.volume_24h_usd
                    .partial_cmp(&b.volume_24h_usd)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Price {
            self.items_mut().sort_by(|a, b| {
                a.price_usd
                    .partial_cmp(&b.price_usd)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Floor {
            self.items_mut().sort_by(|a, b| {
                a.floor_price
                    .partial_cmp(&b.floor_price)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Marked {
            self.items_mut().sort_by(|b, a| a.index.cmp(&b.index));
            self.items_mut().sort_by(|a, b| a.marked.cmp(&b.marked));
        } else {
            return;
        }

        if self.sort_dir != SortDir::UP {
            self.items_mut().reverse();
        }
        self.sort_key = key as u32;
        self.items_changed(0, self.items_len() - 1);
    }

    fn new_price(raw_prices: RawItem) -> Item {
        return Item {
            id: raw_prices.id.into(),
            name: raw_prices.name.into(),
            symbol: raw_prices.symbol.into(),
            rank: raw_prices.rank.parse().unwrap_or(0),
            price_usd: raw_prices.price_usd.parse().unwrap_or(0.0),
            volume_24h_usd: raw_prices.volume_24h_usd.parse().unwrap_or(0.0),
            market_cap_usd: raw_prices.market_cap_usd.parse().unwrap_or(0),
            available_supply: raw_prices.available_supply.parse().unwrap_or(0),
            total_supply: raw_prices.total_supply.parse().unwrap_or(0),
            max_supply: raw_prices.max_supply.parse().unwrap_or(0),
            percent_change_1h: raw_prices.percent_change_1h.parse().unwrap_or(0.0),
            percent_change_24h: raw_prices.percent_change_24h.parse().unwrap_or(0.0),
            percent_change_7d: raw_prices.percent_change_7d.parse().unwrap_or(0.0),
            last_updated: raw_prices.last_updated.parse().unwrap_or(0),
            ..Item::default()
        };
    }

    // 获取私有数据
    fn get_private(&self, symbol: &str) -> Option<&Private> {
        for item in &self.private {
            if item.symbol.to_lowercase() == symbol.to_lowercase() {
                return Some(item);
            }
        }
        return None;
    }

    // 设置关注
    fn set_marked_qml(&mut self, index: usize, marked: bool) {
        if index >= self.items_len() {
            return;
        }

        let mut item = self.items()[index].clone();
        item.marked = marked;
        self.set(index, item);
        self.save_private();
    }

    // 设置地板价格
    fn set_floor_price_qml(&mut self, index: usize, price: f32) {
        if index >= self.items_len() {
            return;
        }

        let mut item = self.items()[index].clone();
        item.floor_price = price;
        self.set(index, item);
        self.save_private();
    }

    // 设置数据url
    fn set_url_qml(&mut self, limit: u32) {
        self.item_max_count = limit;
        *self.url.lock().unwrap() =
            "https://api.alternative.me/v1/ticker/?limit=".to_string() + &limit.to_string();
    }

    // 查找并与第一行交换
    fn search_and_view_at_beginning_qml(&mut self, text: QString) {
        if let Some(index) = self
            .items()
            .iter()
            .position(|a| a.symbol.to_lower() == text.to_lower())
        {
            self.swap_row(0, index);
        }
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
    }

    fn toggle_sort_dir_qml(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    fn set_update_interval_qml(&mut self, interval: u32) {
        self.update_interval
            .store(u32::max(5, interval), AOrdering::SeqCst);
    }
}
