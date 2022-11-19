use super::data::{
    NFTSudoSwapCollectionRawItem, NFTSudoSwapRawItem as RawItem, NTFSudoSwapItem as Item,
};
use super::sort::{SortDir, SudoSwapSortKey as SortKey};
use crate::httpclient;
use crate::utility::Utility;
use ::log::debug;
use cstr::cstr;
use modeldata::*;
use qmetaobject::*;
use reqwest::header::HeaderMap;
use std::cmp::Ordering;
use std::sync::atomic::{AtomicBool, Ordering as AOrdering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
        update_now: AtomicBool,
    }, members_qt: {
        bull_percent: [f32; bull_percent_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
    }, methods_qt: {
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
        refresh_qml: fn(&mut self),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow().url.clone();
    }

    fn update_interval(&self) -> usize {
        return usize::max_value();
    }

    fn update_now(&self) -> bool {
        return self.borrow().update_now.load(AOrdering::SeqCst);
    }

    fn disable_update_now(&self) {
        self.borrow().update_now.store(false, AOrdering::SeqCst);
    }

    fn parse_body(&mut self, text: &str) {
        self.borrow_mut().cache_items(text);
    }

    fn headers(&mut self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("user-agent","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36".parse().unwrap());
        headers.insert("accept", "*/*".parse().unwrap());
        headers
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(
            cstr!("NFTSudoSwapSortKey"),
            1,
            0,
            cstr!("NFTSudoSwapSortKey"),
        );
        self.sort_key = SortKey::OfferTvl as u32;
        self.url = "https://sudoapi.xyz/v1/collections?sort=offer_tvl&desc=true".to_string();
        self.async_update_model();
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
    }

    fn new_item(raw_item: NFTSudoSwapCollectionRawItem) -> Item {
        let eth = 1000_1000_1000_1000_100_f64;
        return Item {
            address: raw_item.address.into(),
            name: raw_item.name.unwrap_or("-".to_string().into()).into(),
            buy_quote: raw_item.buy_quote.unwrap_or(0_f64) / eth,
            sell_quote: raw_item.sell_quote.unwrap_or(0_f64) / eth,
            offer_tvl: raw_item.offer_tvl / eth,
            pool_count: raw_item.pool_count,
            item_count: raw_item.item_count,
        };
    }

    fn update_model(&mut self, _text: String) {
        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }

        self.clear();
        for item in tmp_items.unwrap() {
            self.append(item);
        }

        self.sort_by_key_qml(self.sort_key);
        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
    }

    fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 1, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<RawItem>(text) {
            Ok(raw_item) => {
                let mut v = vec![];

                for item in raw_item.collections {
                    v.push(Self::new_item(item));
                }
                *self.tmp_items.lock().unwrap() = Some(v);
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
        if key == SortKey::Name {
            self.items_mut().sort_by(|a, b| {
                a.name
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.name.to_string().to_lowercase())
            });
        } else if key == SortKey::BuyQuote {
            self.items_mut().sort_by(|a, b| {
                a.buy_quote
                    .partial_cmp(&b.buy_quote)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::SellQuote {
            self.items_mut().sort_by(|a, b| {
                a.sell_quote
                    .partial_cmp(&b.sell_quote)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::OfferTvl {
            self.items_mut().sort_by(|a, b| {
                a.offer_tvl
                    .partial_cmp(&b.offer_tvl)
                    .unwrap_or(Ordering::Less)
            });
        } else if key == SortKey::PoolCount {
            self.items_mut()
                .sort_by(|a, b| a.pool_count.cmp(&b.pool_count));
        } else if key == SortKey::ItemCount {
            self.items_mut()
                .sort_by(|a, b| a.item_count.cmp(&b.item_count));
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
