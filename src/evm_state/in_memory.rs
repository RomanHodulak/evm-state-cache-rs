/// Simple, single-threaded in-memory implementation of [`EvmStateRepository`].
///
/// All data is kept in-memory and accessed from a single thread.
use crate::evm_state::{Account, Address, EvmStateRepository};
use std::collections::HashMap;

/// In-memory single-threaded ideal for testing.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InMemoryEvmStateRepository {
    accounts: HashMap<Address, Account>,
}

impl EvmStateRepository for InMemoryEvmStateRepository {
    fn get(&self, address: &Address) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    fn replace(&mut self, address: Address, account: Account) {
        self.accounts.insert(address, account);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::{H160, U256};

    #[test]
    fn test_account_by_existent_address_from_repository_is_found() {
        let mut repository = InMemoryEvmStateRepository::default();

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
        let repository = InMemoryEvmStateRepository::default();

        let actual_account = repository.get(&Address::from(H160::zero()));

        assert!(
            actual_account.is_none(),
            "Account found but none was present"
        );
    }
}
