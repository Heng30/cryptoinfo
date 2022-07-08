use super::data::{NewsItem as Item, RawNewsItem as RawItem};
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use modeldata::*;
use qmetaobject::*;

modeldata_struct!(Model, Item, members: {
        url: String,
    }, members_qt: {
        up_refresh: [bool; up_refresh_ok],
        page_index: [u32; page_index_changed],
        text: [QString; text_changed],
        update_interval: [u32; update_interval_changed],
        update_now: [bool; update_now_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
    }, methods_qt: {
        update_all_qml: fn(&mut self),
        clear_qml: fn(&mut self),
    }
);

impl Model {
    pub fn init(&mut self) {
        self.update_interval = 3600;
        self.update_now = false;
        self.page_index = 1;
        self.url = "https://api.theblockbeats.info/v29/newsflash/select?page=".to_string();
    }

    pub fn update_text(&mut self, text: String, page_index: u32) {
        if page_index == 1 {
            self.up_refresh = true;
        }

        self.text = text.into();
        self.text_changed();
    }

    fn update_all_qml(&mut self) {
        if self.text.is_empty() {
            return;
        }

        if self.up_refresh {
            self.clear();
            self.up_refresh = false;
            self.up_refresh_ok();
        }

        self.add_item(self.text.to_string().as_str());
        self.update_time = Utility::default().local_time_now_qml(QString::from("%H:%M:%S"));
        self.update_time_changed();
    }

    fn add_item(&mut self, text: &str) {
        let items: RawItem = serde_json::from_str(&text).unwrap_or(RawItem::default());

        if items.code != 200 {
            return;
        }

        let items = items.data.data;
        for item in items.iter() {
            self.append(Item {
                title: item.title.clone().into(),
                content: item.content.clone().into(),
                url: item.url.clone().into(),
                add_time: Utility::default().utc_seconds_to_local_string_qml(
                    item.add_time,
                    "%m-%d %H:%M".to_string().into(),
                ),
            });
        }
    }

    fn clear_qml(&mut self) {
        self.clear();
        self.page_index = 1;
    }

    //https://www.theblockbeats.info/newsflash/v29/newsflash/select?page=4"
    // pub fn parse_hmtl_lvdong(html: &str) -> String {
    //     let mut items = vec![];
    //     let re = Regex::new(r"flash-item-title.*>(.*)</a>.*flash-item-content.*>(.*)<a.*href=(.*)target.*flash-item-time.*>(.*)</div>").unwrap();
    //     for caps in re.captures_iter(html) {
    //         let mut v = vec![];
    //         for i in 0..4 {
    //             v.push("");
    //             if caps.get(i + 1).is_none() {
    //                 break;
    //             }
    //             v[i] = caps.get(i + 1).unwrap().as_str().trim();
    //         }

    // items.push(RawItem {
    //     title: v[0].to_string(),
    //     content: v[1].to_string(),
    //     url: v[2].to_string(),
    //     time: v[3].to_string(),
    // })
    // }

    // match serde_json::to_string_pretty(&items) {
    //     Ok(text) => return text,
    //     _ => "".to_string(),
    // }
    // }
}
