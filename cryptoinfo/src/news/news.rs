use super::data::{NewsItem as Item, RawNewsItem as RawItem};
use crate::httpclient;
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use modeldata::*;
use qmetaobject::*;

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        url: String,
        tmp_items: ItemVec,
    }, members_qt: {
        page_index: [u32; page_index_changed],
        update_now: [bool; update_now_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
        up_refresh_ok,
    }, methods_qt: {
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow().url.clone() + &format!("{}", self.borrow().page_index);
    }

    fn update_interval(&self) -> usize {
        return usize::max_value();
    }

    fn update_now(&self) -> bool {
        return self.borrow().update_now;
    }

    fn disable_update_new(&self) {
        self.borrow_mut().update_now = false;
    }

    fn parse_body(&mut self, text: &str) {
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        self.update_now = false;
        self.page_index = 1;
        self.url = "https://api.theblockbeats.info/v29/newsflash/select?page=".to_string();

        self.async_update_model();
    }

    fn update_model(&mut self, _text: String) {
        if self.page_index == 1 {
            self.clear();
            self.up_refresh_ok();
        }

        let qptr = QBox::new(self);
        for item in qptr.borrow().tmp_items.iter() {
            self.append(item.clone());
        }

        self.page_index += 1;
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
        let items: RawItem = serde_json::from_str(&text).unwrap_or(RawItem::default());

        if items.code != 200 {
            return;
        }

        self.tmp_items.clear();
        let items = items.data.data;
        for item in items.iter() {
            self.tmp_items.push(Item {
                title: item.title.clone().into(),
                content: item.content.clone().into(),
                url: item.url.clone().into(),
                add_time: Utility::utc_seconds_to_local_string(item.add_time, "%m-%d %H:%M").into(),
            });
        }
    }
}
