use super::data::{
    okex::MainAccountRestItem as Item,
    okex_res::{MainAccountDataRest, MainAccountRest as RawItem},
};
use super::okex_headers;
use crate::httpclient;
use crate::utility::Utility;
use ::log::debug;
use modeldata::*;
use qmetaobject::*;
use reqwest::header::HeaderMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        url: String,
        path: String,
        update_now: AtomicBool,
    }, members_qt: {
        update_time: [QString; update_time_changed],
    }, signals_qt: {
    }, methods_qt: {
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
        return self.borrow().update_now.load(Ordering::SeqCst);
    }

    fn disable_update_now(&self) {
        self.borrow().update_now.store(false, Ordering::SeqCst);
    }

    fn parse_body(&mut self, text: &str) {
        self.borrow_mut().cache_items(text);
    }

    fn headers(&mut self) -> HeaderMap {
        okex_headers::get_headers(&self.borrow().path)
    }
}

impl Model {
    pub fn init(&mut self) {
        self.path = "/api/v5/asset/balances".to_string();
        self.url = format!("{}{}", "https://aws.okx.com", self.path);
        self.async_update_model();
    }

    pub fn refresh_qml(&mut self) {
        self.update_now.store(true, Ordering::SeqCst);
    }

    fn new_item(raw_item: MainAccountDataRest) -> Item {
        return Item {
            avail_bal: raw_item.avail_bal.into(),
            frozen_bal: raw_item.frozen_bal.into(),
            ccy: raw_item.ccy.into(),
            bal: raw_item.bal.into(),
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
                if raw_item.data.is_empty() {
                    return;
                }

                let mut tmp_items = self.tmp_items.lock().unwrap();
                *tmp_items = Some(vec![]);

                for item in raw_item.data {
                    if item.bal.parse::<f64>().unwrap_or(0_f64) < 0.0001 {
                        continue;
                    }
                    tmp_items.as_mut().unwrap().push(Self::new_item(item));
                }
            }
            Err(e) => debug!("{:?}", e),
        }
    }
}
