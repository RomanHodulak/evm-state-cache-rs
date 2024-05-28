use crate::evm_state::{Account, Address, EvmStateRepository};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct InMemoryEvmStateRepository {
    accounts: HashMap<Address, Account>,
}

impl EvmStateRepository for InMemoryEvmStateRepository {
    fn get(&mut self, address: &Address) -> Option<&Account> {
        self.accounts.get(address)
    }

    fn replace(&mut self, address: Address, account: Account) {
        self.accounts.insert(address, account);
    }
}
