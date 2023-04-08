use qmetaobject::*;
use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NFTSudoSwapRawItem {
    pub collections: Vec<NFTSudoSwapCollectionRawItem>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NFTSudoSwapCollectionRawItem {
    pub address: String,
    pub name: Option<String>,

    #[serde(deserialize_with = "from_str")]
    pub buy_quote: f64,

    #[serde(deserialize_with = "from_str")]
    pub sell_quote: f64,

    #[serde(deserialize_with = "from_str")]
    pub offer_tvl: f64,
    pub pool_count: Option<i32>,
    pub item_count: Option<i32>,
}

#[derive(QGadget, Clone, Default)]
pub struct NTFSudoSwapItem {
    pub address: qt_property!(QString),
    pub name: qt_property!(QString),
    pub buy_quote: qt_property!(f64),
    pub sell_quote: qt_property!(f64),
    pub offer_tvl: qt_property!(f64),
    pub pool_count: qt_property!(i32),
    pub item_count: qt_property!(i32),
}

#[derive(Deserialize)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrF64 {
    String(String),
    F64(f64),
}

fn from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match StrOrF64::deserialize(deserializer) {
        Ok(w) => match w {
            StrOrF64::String(v) => v.parse::<f64>().unwrap_or_default(),
            StrOrF64::F64(v) => v,
        },
        Err(_) => 0_f64,
    })
}
