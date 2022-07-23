use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(QGadget, Clone, Default)]
pub struct AddressItem {
    pub address: qt_property!(QString),
    pub balance: qt_property!(f64),
    pub percentage: qt_property!(f64),
    pub transactions: qt_property!(u64),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AddressEthDataResultRawItem {
    pub address: String,
    pub balance: String,
    pub percentage: f64,
    pub transactions: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AddressEthDataRawItem {
    pub result: Vec<AddressEthDataResultRawItem>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AddressEthRawItem {
    pub data: AddressEthDataRawItem,
}

