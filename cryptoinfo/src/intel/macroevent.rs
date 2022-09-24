use super::data::{MacroEventItem as Item, RawMacroEventItem as RawItem};
use crate::httpclient;
use crate::utility::Utility;
use ::log::debug;
use chrono::prelude::Local;
use modeldata::*;
use qmetaobject::*;
use std::sync::atomic::{AtomicBool, Ordering as AOrdering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        url: String,
        tmp_items: ItemVec,
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
        let timestamp = Local::now().timestamp();
        let timestamp_24h_before = timestamp - 24 * 3600;
        let timestamp_24h_after = timestamp + 24 * 3600;
        return self.borrow().url.clone()
            + &format!("?start={}&end={}", timestamp_24h_before, timestamp_24h_after);
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
        self.url = "https://api-one-wscn.awtmt.com/apiv1/finance/macrodatas".to_string();
        self.async_update_model();
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
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

        httpclient::download_timer_pro(qptr, 1, cb);
    }

    fn cache_items(&mut self, text: &str) {
        self.add_item(text);
        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
    }

    fn add_item(&mut self, text: &str) {
        match serde_json::from_str::<RawItem>(&text) {
            Ok(items) => {
                let mut v = vec![];
                for item in items.data.items {
                    v.push(Item {
                        country: item.country.into(),
                        title: item.title.into(),
                        unit: item.unit.into(),
                        actual: item.actual.into(),
                        forecast: item.forecast.into(),
                        previous: item.previous.into(),
                        importance: item.importance,
                        public_date: Utility::utc_seconds_to_local_string(
                            item.public_date,
                            "%m-%d %H:%M",
                        )
                        .into(),
                    });
                }
                *self.tmp_items.lock().unwrap() = Some(v);
            }

            Err(e) => debug!("{:?}", e),
        }
    }
}
