use super::data::CryptoFeeItem as Item;
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused)]
use ::log::debug;
use modeldata::*;
use qmetaobject::*;
use regex::Regex;
use std::sync::atomic::{AtomicBool, Ordering as AOrdering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        url: String,
        tmp_items: ItemVec,
        update_now: AtomicBool,
    }, members_qt: {
        update_time: [QString; update_time_changed], //数据更新时间
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
        return self.borrow().update_now.load(AOrdering::SeqCst);
    }

    fn disable_update_now(&self) {
        self.borrow().update_now.store(false, AOrdering::SeqCst);
    }

    fn parse_body(&mut self, text: &str) {
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        self.url = "https://cryptofees.info/".to_string();
        self.async_update_model();
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

    pub fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 10, cb);
    }

    pub fn cache_items(&mut self, text: &str) {
        let new_text = text.replace('\n', "");
        let mut v = vec![];

        let re = Regex::new(
            r#"<div class="jsx-166918656 name">[\s]*<div class="jsx-166918656.*?">(.+?)</div>.+?<div class="jsx-1590841784 amount">(.+?)</div>[\s]*<div class="jsx-1590841784 amount">(.+?)</div>[\s]*<div class="jsx-1590841784 arrow">"#).unwrap();

        for cap in re.captures_iter(&new_text) {
            let groups = (cap.get(1), cap.get(2), cap.get(3));
            match groups {
                (Some(name), Some(fee_1day), Some(fee_7day_avg)) => v.push(Item {
                    name: name.as_str().to_string().into(),
                    fee_1day: fee_1day.as_str().to_string().into(),
                    fee_7day_avg: fee_7day_avg.as_str().to_string().into(),
                }),
                _ => (),
            }
        }

        if !v.is_empty() {
            *self.tmp_items.lock().unwrap() = Some(v);
        }
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
    }
}
