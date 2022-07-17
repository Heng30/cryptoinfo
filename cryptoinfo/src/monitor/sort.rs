use qmetaobject::*;

/// 搜索方向
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
}

impl From<u32> for SortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SortKey::TxValue,
            _ => return SortKey::Unknown,
        }
    }
}
