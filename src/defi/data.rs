use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

/// 从json文件中解析出来的条目对象
#[derive(Serialize, Deserialize, Debug)]
pub struct RawItem {
    pub name: String,
    pub symbol: String,

    #[serde(default)]
    pub tvl: f64,

    #[serde(default)]
    pub change_1h: Option<f64>,

    #[serde(default)]
    pub change_1d: Option<f64>,

    #[serde(default)]
    pub change_7d: Option<f64>,

    #[serde(default)]
    pub mcap: f64,

    #[serde(default)]
    pub staking: f64,
}

/// 与qml交互的条目对象
#[derive(QGadget, Clone, Default)]
pub struct DefiItem {
    pub index: qt_property!(i32),
    pub name: qt_property!(QString),
    pub symbol: qt_property!(QString),
    pub tvl: qt_property!(f64),
    pub staking: qt_property!(f64),
    pub percent_change_1h: qt_property!(f64),
    pub percent_change_24h: qt_property!(f64),
    pub percent_change_7d: qt_property!(f64),
    pub market_cap_usd: qt_property!(f64),
}
