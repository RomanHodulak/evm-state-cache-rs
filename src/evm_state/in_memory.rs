/// Simple, single-threaded in-memory implementation of [`EvmStateRepository`].
///
/// All data is kept in-memory and accessed from a single thread.
use crate::evm_state::{Account, Address, EvmStateRepository};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct InMemoryEvmStateRepository {
    accounts: HashMap<Address, Account>,
}

impl EvmStateRepository for InMemoryEvmStateRepository {
    fn get(&mut self, address: &Address) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    fn replace(&mut self, address: Address, account: Account) {
        self.accounts.insert(address, account);
    }
}
