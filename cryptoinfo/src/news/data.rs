use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawNewsItem {
    pub title: String,
    pub content: String,
    pub url: String,
    pub time: String,
}

#[derive(QGadget, Clone, Default)]
pub struct NewsItem {
    pub title: qt_property!(QString),
    pub content: qt_property!(QString),
    pub url: qt_property!(QString),
    pub time: qt_property!(QString),
}
