use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

/// 保存到本地的有关价格条目的私有数据
#[derive(Serialize, Deserialize, Debug)]
pub struct Private {
    pub symbol: String,
    pub marked: bool,
    pub floor_price: f32,
}

/// 从json文件中解析出来的条目对象
#[derive(Serialize, Deserialize, Debug)]
pub struct RawItem {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub rank: String,
    pub price_usd: String,
    pub market_cap_usd: String,
    pub available_supply: String,
    pub total_supply: String,
    pub max_supply: String,
    pub percent_change_1h: String,
    pub percent_change_24h: String,
    pub percent_change_7d: String,
    pub last_updated: String,

    #[serde(rename(serialize = "24h_volume_usd", deserialize = "24h_volume_usd"))]
    pub volume_24h_usd: String,
}

/// 与qml交互的条目对象
#[derive(QGadget, Clone, Default)]
pub struct PItem {
    pub index: qt_property!(i32),
    pub marked: qt_property!(bool),
    pub floor_price: qt_property!(f32),

    pub id: qt_property!(QString),
    pub name: qt_property!(QString),
    pub symbol: qt_property!(QString),
    pub rank: qt_property!(u32),
    pub price_usd: qt_property!(f32),
    pub market_cap_usd: qt_property!(i64),
    pub available_supply: qt_property!(i64),
    pub total_supply: qt_property!(i64),
    pub max_supply: qt_property!(i64),
    pub percent_change_1h: qt_property!(f32),
    pub percent_change_24h: qt_property!(f32),
    pub percent_change_7d: qt_property!(f32),
    pub volume_24h_usd: qt_property!(f64),
    pub last_updated: qt_property!(i64),
}
