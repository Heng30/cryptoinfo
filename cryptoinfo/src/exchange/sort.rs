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
    Balance = 2,
    Income = 3,
    Rate = 4,
}

impl From<u32> for SortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SortKey::Name,
            2 => return SortKey::Balance,
            3 => return SortKey::Income,
            4 => return SortKey::Rate,
            _ => return SortKey::Unknown,
        }
    }
}
