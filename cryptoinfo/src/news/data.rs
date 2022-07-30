use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawNewsItemDataData {
    pub title: String,
    pub content: String,
    pub url: String,
    pub add_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawNewsItemData {
    pub data: Vec<RawNewsItemDataData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawNewsItem {
    pub code: u32,
    pub message: String,
    pub data: RawNewsItemData,
}

#[derive(QGadget, Clone, Default)]
pub struct NewsItem {
    pub title: qt_property!(QString),
    pub content: qt_property!(QString),
    pub url: qt_property!(QString),
    pub add_time: qt_property!(QString),
}
