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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawMacroEventItem {
    pub code: i32,
    pub message: String,
    pub data: RawMacroEventItemData,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawMacroEventItemData {
    pub items: Vec<RawMacroEventItemDataItem>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawMacroEventItemDataItem {
    pub public_date: i64,
    pub country: String,
    pub title: String,
    pub unit: String,
    pub importance: u32,
    pub actual: String,
    pub forecast: String,
    pub previous: String,
}

#[derive(QGadget, Clone, Default)]
pub struct MacroEventItem {
    pub public_date: qt_property!(QString),
    pub country: qt_property!(QString),
    pub title: qt_property!(QString),
    pub unit: qt_property!(QString),
    pub importance: qt_property!(u32),
    pub actual: qt_property!(QString),
    pub forecast: qt_property!(QString),
    pub previous: qt_property!(QString),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawMacroNewsItem {
    pub code: i32,
    pub message: String,
    pub data: RawMacroNewsItemData,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawMacroNewsItemData {
    pub next_cursor: String,
    pub items: Vec<RawMacroNewsItemDataItem>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawMacroNewsItemDataItem {
    pub title: String,
    pub content_text: String,
    pub uri: String,
    pub display_time: i64,
    pub score: i32,
}

#[derive(QGadget, Clone, Default)]
pub struct MacroNewsItem {
    pub title: qt_property!(QString),
    pub content: qt_property!(QString),
    pub url: qt_property!(QString),
    pub add_time: qt_property!(QString),
    pub score: qt_property!(i32),
}
