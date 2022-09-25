use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawNotifyItem {
    pub timestamp: String,
    pub level: i32,
    pub module: String,
    pub content: String,
}

#[derive(QGadget, Clone, Default)]
pub struct NotifyItem {
    pub timestamp: qt_property!(QString),
    pub level: qt_property!(i32),
    pub module: qt_property!(QString),
    pub content: qt_property!(QString),
}
