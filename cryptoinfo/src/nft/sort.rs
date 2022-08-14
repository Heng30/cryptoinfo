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

impl From<u32> for SortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SortKey::Name,
            2 => return SortKey::OneDayVolume,
            3 => return SortKey::OneDayChange,
            4 => return SortKey::SevenDayChange,
            5 => return SortKey::TotalVolume,
            6 => return SortKey::TotalSales,
            7 => return SortKey::TotalSupply,
            8 => return SortKey::NumOwners,
            9 => return SortKey::FloorPrice,
            _ => return SortKey::Unknown,
        }
    }
}
