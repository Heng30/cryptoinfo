use super::data::{NewsItem as Item, RawNewsItem as RawItem};
use crate::utility::Utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use modeldata::*;
use qmetaobject::*;
use regex::Regex;

modeldata_struct!(Model, Item, members: {
        url: String,
    }, members_qt: {
        text: [QString; text_changed],
        update_interval: [u32; update_interval_changed],
        update_now: [bool; update_now_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
    }, methods_qt: {
        update_all_qml: fn(&mut self),
    }
);

impl Model {
    pub fn init(&mut self) {
        self.update_interval = 10;
        self.update_now = false;
        self.url = "https://www.theblockbeats.info/newsflash".to_string();
    }

    pub fn update_text(&mut self, text: String) {
        self.text = text.into();
        self.text_changed();
    }

    fn update_all_qml(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.to_string().clone();
        self.reset(&text);
        self.update_time = Utility::default().local_time_now_qml(QString::from("%H:%M:%S"));
        self.update_time_changed();
    }

    fn add_item(&mut self, _index: usize, raw_item: &RawItem) {
        let item = Item {
            title: raw_item.title.clone().into(),
            content: raw_item.content.clone().into(),
            url: raw_item.content.clone().into(),
            time: raw_item.time.clone().into(),
        };
        self.append(item);
    }

    fn reset(&mut self, text: &str) {
        let items: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);

        if items.len() > 0 {
            self.clear();
        }

        for (i, item) in items.iter().enumerate() {
            self.add_item(i, &item);
        }
    }

    pub fn parse_hmtl_lvdong(html: &str) -> String {
        let mut items = vec![];
        let re = Regex::new(r"flash-item-title.*>(.*)</a>.*flash-item-content.*>(.*)<a.*href=(.*)target.*flash-item-time.*>(.*)</div>").unwrap();
        for caps in re.captures_iter(html) {
            let mut v = vec![];
            for i in 0..4 {
                v.push("");
                if caps.get(i + 1).is_none() {
                    break;
                }
                v[i] = caps.get(i + 1).unwrap().as_str().trim();
            }

            items.push(RawItem {
                title: v[0].to_string(),
                content: v[1].to_string(),
                url: v[2].to_string(),
                time: v[3].to_string(),
            })
        }

        match serde_json::to_string_pretty(&items) {
            Ok(text) => return text,
            _ => "".to_string(),
        }
    }
}
