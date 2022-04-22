use qmetaobject::*;

/// 搜索方向
#[derive(Debug, PartialEq)]
pub enum SortDir {
    UP,
    DOWN,
}

impl Default for SortDir {
    fn default() -> Self {
        return SortDir::UP;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
pub enum SortKey {
    Index = 1,
    Name = 2,
    Symbol = 3,
    TVL = 4,
    Staking = 5,
    MarketCap = 6,
    Per1H = 7,
    Per24H = 8,
    Per7D = 9,
}

impl From<u32> for SortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SortKey::Index,
            2 => return SortKey::Name,
            3 => return SortKey::Symbol,
            4 => return SortKey::TVL,
            5 => return SortKey::Staking,
            6 => return SortKey::MarketCap,
            7 => return SortKey::Per1H,
            8 => return SortKey::Per24H,
            9 => return SortKey::Per7D,
            _ => return SortKey::Index,
        }
    }
}
