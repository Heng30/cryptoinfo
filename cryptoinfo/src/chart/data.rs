use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawChainTVLItem {
    pub date: String,

    #[serde(rename(deserialize = "totalLiquidityUSD"))]
    pub tvl: f64,
}

#[derive(QGadget, Clone, Default)]
pub struct ChainTVLItem {
    pub index: qt_property!(i32),
    pub second: qt_property!(u64),
    pub tvl: qt_property!(u64),
}
