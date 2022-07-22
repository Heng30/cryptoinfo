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
    Balance = 1,
    Percentage = 2,
    Transactions = 3,
}

impl From<u32> for SortKey {
    fn from(item: u32) -> Self {
        match item {
            1 => return SortKey::Balance,
            2 => return SortKey::Percentage,
            3 => return SortKey::Transactions,
            _ => return SortKey::Unknown,
        }
    }
}
