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
pub enum ProtocolSortKey {
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

impl From<u32> for ProtocolSortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return ProtocolSortKey::Index,
            2 => return ProtocolSortKey::Name,
            3 => return ProtocolSortKey::Symbol,
            4 => return ProtocolSortKey::TVL,
            5 => return ProtocolSortKey::Staking,
            6 => return ProtocolSortKey::MarketCap,
            7 => return ProtocolSortKey::Per1H,
            8 => return ProtocolSortKey::Per24H,
            9 => return ProtocolSortKey::Per7D,
            _ => return ProtocolSortKey::Index,
        }
    }
}
