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
pub struct PriceItem {
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawGreed {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FearGreed {
    pub data: Vec<RawGreed>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawMarket {
    pub total_market_cap_usd: i64,
    pub total_24h_volume_usd: i64,
    pub bitcoin_percentage_of_market_cap: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawEthGas {
    #[serde(rename(serialize = "safeLow", deserialize = "safeLow"))]
    pub low: u32,

    pub average: u32,
    pub fast: u32,

    #[serde(rename(serialize = "safeLowWait", deserialize = "safeLowWait"))]
    pub low_wait: f32,

    #[serde(rename(serialize = "avgWait", deserialize = "avgWait"))]
    pub average_wait: f32,

    #[serde(rename(serialize = "fastWait", deserialize = "fastWait"))]
    pub fast_wait: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawEthBurned {
    pub total_burned: f64,
    pub burn_rate_1_h: f64,
    pub burn_rate_24_h: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawBTCStats {
    pub minutes_between_blocks: f32,
    pub n_blocks_total: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawLongShortItem {
    pub symbol: String,

    #[serde(rename(serialize = "longRate", deserialize = "longRate"))]
    pub long_rate: f32,

    #[serde(rename(serialize = "longVolUsd", deserialize = "longVolUsd"))]
    pub long_vol_usd: f64,

    #[serde(rename(serialize = "shortRate", deserialize = "shortRate"))]
    pub short_rate: f32,

    #[serde(rename(serialize = "shortVolUsd", deserialize = "shortVolUsd"))]
    pub short_vol_usd: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawLongShort {
    pub code: i32,
    pub success: bool,
    pub data: Vec<RawLongShortItem>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawOtcItem {
    pub usd: String,
    pub usdt: String,
    pub datetime: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawOtc {
    pub code: i32,
    pub msg: String,
    pub data: Vec<RawOtcItem>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawBtcInfoHashes {
    #[serde(rename(serialize = "globalHashes", deserialize = "globalHashes"))]
    pub global_hashes: String,

    #[serde(rename(
        serialize = "globalHashesPercentChange24h",
        deserialize = "globalHashesPercentChange24h"
    ))]
    pub global_hashes_percent_change_24h: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawBtcInfoData {
    pub hashes: RawBtcInfoHashes,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawBtcInfo {
    pub code: i32,
    pub msg: String,
    pub data: RawBtcInfoData,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawBtcMa730Item {
    #[serde(rename(serialize = "createTime", deserialize = "createTime"))]
    pub create_time: u64,

    pub price: f64,

    #[serde(rename(serialize = "mA730Mu5", deserialize = "mA730Mu5"))]
    pub ma730_mu5: f64,

    #[serde(rename(serialize = "mA730", deserialize = "mA730"))]
    pub ma730: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RawBtcMa730 {
    pub code: i32,
    pub msg: String,
    pub data: Vec<RawBtcMa730Item>,
}
