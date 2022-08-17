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

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        url: String,
        path: String,
    }, members_qt: {
        update_now: [bool; update_now_changed],
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
        let _ = self.mutex.lock().unwrap();
        self.update_now = true;
    }

    fn new_item(raw_item: &MainAccountDataRest) -> Item {
        return Item {
            avail_bal: raw_item.avail_bal.clone().into(),
            frozen_bal: raw_item.frozen_bal.clone().into(),
            ccy: raw_item.ccy.clone().into(),
            bal: raw_item.bal.clone().into(),
        };
    }

    fn update_model(&mut self, _text: String) {
        {
            let _ = self.mutex.lock().unwrap();
            self.clear();
            let qptr = QBox::new(self);
            for item in qptr.borrow().tmp_items.iter() {
                self.append(item.clone());
            }
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
                self.tmp_items.clear();

                for item in raw_item.data.iter() {
                    if item.bal.parse::<f64>().unwrap_or(0_f64) < 0.0001 {
                        continue;
                    }
                    self.tmp_items.push(Self::new_item(&item));
                }
            }
            Err(e) => debug!("{:?}", e),
        }
    }
}
