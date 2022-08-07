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
pub enum McapSortKey {
    Unknown = 0,
    Name = 1,
    Symbol = 2,
    Circulating = 3,
    Price = 4,
    Index = 5,
    Source = 6,
}

impl From<u32> for McapSortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return McapSortKey::Name,
            2 => return McapSortKey::Symbol,
            3 => return McapSortKey::Circulating,
            4 => return McapSortKey::Price,
            5 => return McapSortKey::Index,
            6 => return McapSortKey::Source,
            _ => return McapSortKey::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
pub enum ChainSortKey {
    Unknown = 0,
    Name = 1,
    Symbol = 2,
    Circulating = 3,
    Index = 4,
}

impl From<u32> for ChainSortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return ChainSortKey::Name,
            2 => return ChainSortKey::Symbol,
            3 => return ChainSortKey::Circulating,
            4 => return ChainSortKey::Index,
            _ => return ChainSortKey::Unknown,
        }
    }
}
