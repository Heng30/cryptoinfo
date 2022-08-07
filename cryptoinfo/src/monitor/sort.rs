use qmetaobject::*;

#[derive(Debug, PartialEq)]
pub enum SortDir {
    UP,
    DOWN,
}

impl Default for SortDir {
    fn default() -> Self {
        return SortDir::DOWN;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
pub enum SortKey {
    Unknown = 0,
    TxValue = 1,
    BlockTime = 2,
}

impl From<u32> for SortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SortKey::TxValue,
            2 => return SortKey::BlockTime,
            _ => return SortKey::Unknown,
        }
    }
}
