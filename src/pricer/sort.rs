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
    Marked = 1,
    Index = 2,
    Symbol = 3,
    Price = 4,
    Per24H = 5,
    Per7D = 6,
    Volume24H = 7,
    Floor = 8,
}

impl From<u32> for SortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SortKey::Marked,
            2 => return SortKey::Index,
            3 => return SortKey::Symbol,
            4 => return SortKey::Price,
            5 => return SortKey::Per24H,
            6 => return SortKey::Per7D,
            7 => return SortKey::Volume24H,
            8 => return SortKey::Floor,
            _ => return SortKey::Marked,
        }
    }
}
