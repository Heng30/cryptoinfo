use super::data::{EthTokenDataResultRawItem, EthTokenItem as Item, EthTokenRawItem as RawItem};
use super::sort::{EthTokenSortKey as SortKey, SortDir};
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use qmetaobject::*;
use std::cmp::Ordering;

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
        page: u32,
    }, members_qt: {
        update_now: [bool; update_now_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
        refresh_ok,
    }, methods_qt: {
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
        up_refresh_qml: fn(&mut self),
        down_refresh_qml: fn(&mut self),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return format!("{}{}", &self.borrow().url, self.borrow().get_page());
    }

    fn update_interval(&self) -> usize {
        return usize::max_value();
    }

    fn update_now(&self) -> bool {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        return self.borrow().update_now;
    }

    fn disable_update_now(&self) {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        self.borrow_mut().update_now = false;
    }

    fn parse_body(&mut self, text: &str) {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(cstr!("EthTokenSortKey"), 1, 0, cstr!("EthTokenSortKey"));
        self.sort_key = SortKey::MarketCap as u32;
        self.sort_dir = SortDir::DOWN;
        self.page = 1;
        self.url = "https://api.yitaifang.com/index/tokens/?page=".to_string();
        self.async_update_model();
    }

    fn get_page(&self) -> u32 {
        let _ = self.mutex.lock().unwrap();
        return self.page;
    }

    fn up_refresh_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        self.page = 1;
        self.update_now = true;
    }

    fn down_refresh_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        self.update_now = true;
    }

    fn new_item(raw_item: &EthTokenDataResultRawItem) -> Item {
        return Item {
            uptime: raw_item.uptime.clone().into(),
            name: raw_item.en.clone().into(),
            symbol: raw_item.name.clone().into(),
            address: raw_item.address.clone().into(),
            pubdate: raw_item.pubdate.clone().into(),
            price_usd: raw_item.price_usd,
            market_cap_usd: raw_item.market_cap_usd,
            volume_usd: raw_item.volume_usd,
            circulation_quantity: raw_item.circulation_quantity,
            issue_quantity: raw_item.issue_quantity,
        };
    }

    fn update_model(&mut self, _text: String) {
        {
            let _ = self.mutex.lock().unwrap();
            if self.page == 1 {
                self.clear();
            }
            let qptr = QBox::new(self);
            for item in qptr.borrow().tmp_items.iter() {
                self.append(item.clone());
            }
            self.page += 1;
        }

        self.sort_by_key_qml(self.sort_key as u32);
        self.refresh_ok();
        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
    }

    fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 5, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<RawItem>(text) {
            Ok(raw_item) => {
                if raw_item.data.result.is_empty() {
                    return;
                }
                self.tmp_items.clear();

                for item in raw_item.data.result.iter() {
                    self.tmp_items.push(Self::new_item(&item));
                }
            }
            Err(e) => debug!("{:?}", e),
        }
    }

    fn toggle_sort_dir_qml(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    fn sort_by_key_qml(&mut self, key: u32) {
        if self.items_is_empty() {
            return;
        }

        let key: SortKey = key.into();
        if key == SortKey::Price {
            self.items_mut().sort_by(|a, b| {
                a.price_usd
                    .partial_cmp(&b.price_usd)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::MarketCap {
            self.items_mut().sort_by(|a, b| {
                a.market_cap_usd
                    .partial_cmp(&b.market_cap_usd)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Volumn {
            self.items_mut().sort_by(|a, b| {
                a.volume_usd
                    .partial_cmp(&b.volume_usd)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::Name {
            self.items_mut()
                .sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));
        } else if key == SortKey::Symbol {
            self.items_mut()
                .sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
        } else if key == SortKey::CirQuantity {
            self.items_mut().sort_by(|a, b| {
                a.circulation_quantity
                    .partial_cmp(&b.circulation_quantity)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::IssueQuantity {
            self.items_mut().sort_by(|a, b| {
                a.issue_quantity
                    .partial_cmp(&b.issue_quantity)
                    .unwrap_or(Ordering::Less)
            });
        } else {
            return;
        }

        if self.sort_dir != SortDir::UP {
            self.items_mut().reverse();
        }
        self.sort_key = key as u32;
        self.items_changed(0, self.items_len() - 1);
    }
}
