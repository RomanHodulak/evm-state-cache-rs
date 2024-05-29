/// Concurrent, in-memory implementation of [`EvmStateRepository`].
///
/// All data is kept in-memory and accessed from a single thread.
use crate::evm_state::{Account, Address, EvmStateRepository};
use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct ConcurrentInMemoryEvmStateRepository {
    accounts: DashMap<Address, Account>,
}

impl EvmStateRepository for ConcurrentInMemoryEvmStateRepository {
    fn get(&mut self, address: &Address) -> Option<Account> {
        self.accounts.get(address).map(|v| v.clone())
    }

    fn replace(&mut self, address: Address, account: Account) {
        self.accounts.insert(address, account);
    }
}
