use qmetaobject::*;

/// 搜索方向
#[derive(Debug, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
pub enum TvlSortKey {
    Index = 1,
    Name = 2,
    Symbol = 3,
    TVL = 4,
}

impl From<u32> for TvlSortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return TvlSortKey::Index,
            2 => return TvlSortKey::Name,
            3 => return TvlSortKey::Symbol,
            4 => return TvlSortKey::TVL,
            _ => return TvlSortKey::Index,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
pub enum YieldSortKey {
    Unknown = 0,
    Index = 1,
    Chain = 2,
    Symbol = 3,
    Tvl = 4,
    Project = 5,
    Apy = 6,
    Pool = 7,
    StableCoin = 8,
    Exposure = 9,
}

impl From<u32> for YieldSortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return YieldSortKey::Index,
            2 => return YieldSortKey::Chain,
            3 => return YieldSortKey::Symbol,
            4 => return YieldSortKey::Tvl,
            5 => return YieldSortKey::Project,
            6 => return YieldSortKey::Apy,
            7 => return YieldSortKey::Pool,
            8 => return YieldSortKey::StableCoin,
            9 => return YieldSortKey::Exposure,
            _ => return YieldSortKey::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
pub enum EthTokenSortKey {
    Unknown = 0,
    Name = 1,
    Symbol = 2,
    Pubdate = 3,
    Price = 4,
    MarketCap = 5,
    Volumn = 6,
    CirQuantity = 7,
    IssueQuantity = 8,
}

impl From<u32> for EthTokenSortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return EthTokenSortKey::Name,
            2 => return EthTokenSortKey::Symbol,
            3 => return EthTokenSortKey::Pubdate,
            4 => return EthTokenSortKey::Price,
            5 => return EthTokenSortKey::MarketCap,
            6 => return EthTokenSortKey::Volumn,
            7 => return EthTokenSortKey::CirQuantity,
            8 => return EthTokenSortKey::IssueQuantity,
            _ => return EthTokenSortKey::Unknown,
        }
    }
}
