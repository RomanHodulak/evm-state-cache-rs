mod cached;
mod in_memory;

use primitive_types::U256;

pub type Address = [u8; 20];

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    nonce: u64,
    balance: U256,
    code_hash: U256,
    storage_root: U256,
}

pub trait EvmStateRepository {
    fn get(&mut self, address: &Address) -> Option<&Account>;
    fn replace(&mut self, address: Address, account: Account);
}
