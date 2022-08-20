use super::data::{NewsItem as Item, RawNewsItem as RawItem};
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use modeldata::*;
use qmetaobject::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering as AOrdering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        url: String,
        tmp_items: ItemVec,
        page_index: AtomicU32,
        update_now: AtomicBool,
    }, members_qt: {
        update_time: [QString; update_time_changed],
    }, signals_qt: {
        up_refresh_ok,
    }, methods_qt: {
        refresh_qml: fn(&mut self),
        reset_page_index_qml: fn(&mut self),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow().url.clone()
            + &format!("{}", self.borrow().page_index.load(AOrdering::SeqCst));
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
        self.page_index = AtomicU32::new(1);
        self.url = "https://api.theblockbeats.info/v29/newsflash/select?page=".to_string();

        self.async_update_model();
    }

    fn reset_page_index_qml(&mut self) {
        self.page_index.store(1, AOrdering::SeqCst);
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
    }

    fn update_model(&mut self, _text: String) {
        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }

        if self.page_index.fetch_add(1, AOrdering::SeqCst) == 1 {
            self.clear();
            self.up_refresh_ok();
        }

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
                if items.code != 200 {
                    return;
                }

                let mut v = vec![];
                for item in items.data.data {
                    v.push(Item {
                        title: item.title.into(),
                        content: item.content.into(),
                        url: item.url.into(),
                        add_time: Utility::utc_seconds_to_local_string(
                            item.add_time,
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
