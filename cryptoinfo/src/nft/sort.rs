use qmetaobject::*;

/// 搜索方向
#[derive(Debug, PartialEq, Eq)]
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
pub enum GemSortKey {
    Unknown = 0,
    Name = 1,
    OneDayVolume = 2,
    OneDayChange = 3,
    SevenDayChange = 4,
    TotalVolume = 5,
    TotalSales = 6,
    TotalSupply = 7,
    NumOwners = 8,
    FloorPrice = 9,
}

impl From<u32> for GemSortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return GemSortKey::Name,
            2 => return GemSortKey::OneDayVolume,
            3 => return GemSortKey::OneDayChange,
            4 => return GemSortKey::SevenDayChange,
            5 => return GemSortKey::TotalVolume,
            6 => return GemSortKey::TotalSales,
            7 => return GemSortKey::TotalSupply,
            8 => return GemSortKey::NumOwners,
            9 => return GemSortKey::FloorPrice,
            _ => return GemSortKey::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
pub enum SudoSwapSortKey {
    Unknown = 0,
    Name = 1,
    BuyQuote = 2,
    SellQuote = 3,
    PoolCount = 4,
    ItemCount = 5,
    OfferTvl = 6,
}

impl From<u32> for SudoSwapSortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SudoSwapSortKey::Name,
            2 => return SudoSwapSortKey::BuyQuote,
            3 => return SudoSwapSortKey::SellQuote,
            4 => return SudoSwapSortKey::PoolCount,
            5 => return SudoSwapSortKey::ItemCount,
            6 => return SudoSwapSortKey::OfferTvl,
            _ => return SudoSwapSortKey::Unknown,
        }
    }
}
