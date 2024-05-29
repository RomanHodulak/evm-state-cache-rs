/// Concurrent, in-memory implementation of [`EvmStateRepository`].
///
/// All data is kept in-memory and can be accessed from a multiple threads concurrently.
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

impl ConcurrentInMemoryEvmStateRepository {
    pub fn new() -> Self {
        Self {
            accounts: DashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::{H160, U256};

    #[test]
    fn test_account_by_existent_address_from_repository_is_found() {
        let mut repository = ConcurrentInMemoryEvmStateRepository::new();

        repository.replace(
            Address::from(H160::zero()),
            Account::new(0, U256::zero(), U256::zero(), U256::zero()),
        );

        let actual_account = repository.get(&Address::from(H160::zero()));
        let expected_account = Account::new(0, U256::zero(), U256::zero(), U256::zero());

        assert!(actual_account.is_some(), "Account not found");

        let actual_account = actual_account.unwrap();

        assert_eq!(expected_account, actual_account);
    }

    #[test]
    fn test_account_by_non_existent_address_from_repository_is_not_found() {
        let mut repository = ConcurrentInMemoryEvmStateRepository::new();

        let actual_account = repository.get(&Address::from(H160::zero()));

        assert!(
            actual_account.is_none(),
            "Account found but none was present"
        );
    }
}
