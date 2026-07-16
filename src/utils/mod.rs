pub mod account;

pub mod print;
pub use crate::println;

pub struct Shared {
    pub accounts: Vec<account::Account>,
}

impl Shared {
    pub fn new() -> Self {
        Shared {
            accounts: Self::get_storage_accounts(),
        }
    }
}
