use crate::evm_state::{Account, Address, EvmStateRepository};
use primitive_types::U256;
use revm::primitives::AccountInfo;
use revm::{Database, DatabaseCommit};
use std::collections::HashMap;

/// Implements [`EvmStateRepository`] that accesses a [`Database`] used by [`revm`].
#[derive(Debug, Clone, PartialEq)]
pub struct RevmStateRepository<D: Database + DatabaseCommit> {
    database: D,
}

impl From<AccountInfo> for Account {
    fn from(value: AccountInfo) -> Self {
        Self {
            nonce: value.nonce,
            balance: U256::from_little_endian(value.balance.as_le_slice()),
            code_hash: U256::from_little_endian(value.code_hash.as_slice()),
            storage_root: Default::default(),
        }
    }
}

impl From<Account> for AccountInfo {
    fn from(value: Account) -> Self {
        Self {
            nonce: value.nonce,
            balance: revm::primitives::U256::from_limbs_slice(&value.balance.0[..]),
            code_hash: revm::primitives::B256::from_slice(
                value.code_hash.0.map(|v| v.to_le_bytes()).as_flattened(),
            ),
            code: None,
        }
    }
}

impl From<Account> for revm::primitives::Account {
    fn from(value: Account) -> Self {
        Self {
            info: value.into(),
            storage: Default::default(),
            status: revm::primitives::state::AccountStatus::Touched,
        }
    }
}

impl<D: Database + DatabaseCommit> EvmStateRepository for RevmStateRepository<D> {
    fn get(&mut self, address: &Address) -> Option<Account> {
        self.database
            .basic(revm::primitives::Address::from(address))
            .ok()
            .flatten()
            .map(Into::into)
    }

    fn replace(&mut self, address: Address, account: Account) {
        self.database.commit({
            let mut map = HashMap::new();
            map.insert(address.into(), account.into());
            map
        });
    }
}

impl<D: Database + DatabaseCommit> RevmStateRepository<D> {
    pub fn new(database: D) -> Self {
        Self { database }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::H160;
    use revm::InMemoryDB;

    #[test]
    fn test_account_by_existent_address_from_repository_is_found() {
        let mut repository = RevmStateRepository::new(InMemoryDB::default());

        repository.replace(
            Address::from(H160::zero()),
            Account::new(0, U256::zero(), U256::zero(), U256::zero()),
        );

        let actual_account = repository.get(&Address::from(H160::zero()));
        let expected_account = Account::new(
            0,
            U256::zero(),
            U256::from_dec_str(
                "50949722399999162638671808714117529995988557342659710791111524410371378434757",
            )
            .unwrap(),
            U256::zero(),
        );

        assert!(actual_account.is_some(), "Account not found");

        let actual_account = actual_account.unwrap();

        assert_eq!(expected_account, actual_account);
    }

    #[test]
    fn test_account_by_non_existent_address_from_repository_is_not_found() {
        let mut repository = RevmStateRepository::new(InMemoryDB::default());

        let actual_account = repository.get(&Address::from(H160::zero()));

        assert!(
            actual_account.is_none(),
            "Account found but none was present"
        );
    }
}
