use super::data::{MacroNewsItem as Item, RawMacroNewsItem as RawItem};
use crate::httpclient;
use crate::utility::Utility;
use ::log::debug;
use modeldata::*;
use qmetaobject::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering as AOrdering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        url: String,
        tmp_items: ItemVec,
        cursor: AtomicU64,
        next_cursor: AtomicU64,
        update_now: AtomicBool,
    }, members_qt: {
        update_time: [QString; update_time_changed],
    }, signals_qt: {
        up_refresh_ok,
    }, methods_qt: {
        refresh_qml: fn(&mut self),
        reset_cursor_qml: fn(&mut self),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        let cursor = self.borrow().cursor.load(AOrdering::SeqCst);
        if cursor <= 0 {
            return self.borrow().url.clone() +
                "?channel=global-channel&client=pc&limit=20&first_page=true&accept=live%2Cvip-live";
        } else {
            return self.borrow().url.clone() +
                &format!("?channel=global-channel&client=pc&limit=20&cursor={}&first_page=false&accept=live%2Cvip-live", cursor);
        };
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
        self.url = "https://api-one-wscn.awtmt.com/apiv1/content/lives".to_string();

        self.async_update_model();
    }

    fn reset_cursor_qml(&mut self) {
        self.cursor.store(0, AOrdering::SeqCst);
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
    }

    fn update_model(&mut self, _text: String) {
        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }

        if self.cursor.load(AOrdering::SeqCst) == 0 {
            self.clear();
            self.up_refresh_ok();
        }

        for item in tmp_items.unwrap() {
            self.append(item);
        }

        self.cursor
            .store(self.next_cursor.load(AOrdering::SeqCst), AOrdering::SeqCst);
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
                        title: item.title.into(),
                        content: item.content_text.into(),
                        url: item.uri.into(),
                        score: item.score,
                        add_time: Utility::utc_seconds_to_local_string(
                            item.display_time,
                            "%m-%d %H:%M",
                        )
                        .into(),
                    });
                }
                *self.tmp_items.lock().unwrap() = Some(v);
                self.next_cursor.store(
                    items.data.next_cursor.parse::<u64>().unwrap_or(0_u64),
                    AOrdering::SeqCst,
                );
            }

            Err(e) => debug!("{:?}", e),
        }
    }
}
