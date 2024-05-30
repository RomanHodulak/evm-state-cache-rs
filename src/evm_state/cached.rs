/// A cached implementation of [`EvmStateRepository`].
///
/// Wraps a different implementation of [`EvmStateRepository`] and adds a caching layer on top
/// of it. Primarily, the data is read from cache.
use crate::cache::Cache;
use crate::evm_state::{Account, Address, EvmStateRepository};

/// An [`EvmStateRepository`] that uses a different repository to access the data and adds a layer
/// of [`Cache`] on top of it.
///
/// This implementation is capable of working while primarily keeping the cache updated and
/// accessed first, before the underlying repository.  
pub struct CachedEvmStateRepository<InnerRepository: EvmStateRepository, C: Cache<Address, Account>>
{
    cache: C,
    inner: InnerRepository,
}

impl<InnerRepository: EvmStateRepository, C: Cache<Address, Account>> EvmStateRepository
    for CachedEvmStateRepository<InnerRepository, C>
{
    fn get(&mut self, address: &Address) -> Option<Account> {
        if !self.cache.contains(address) {
            self.cache.write(*address, self.inner.get(address)?.clone());
        }

        self.cache.read(address)
    }

    fn replace(&mut self, address: Address, account: Account) {
        self.inner.replace(address, account.clone());
        self.cache.write(address, account);
    }
}

impl<InnerRepository: EvmStateRepository, C: Cache<Address, Account>>
    CachedEvmStateRepository<InnerRepository, C>
{
    pub fn new(repository: InnerRepository, cache: C) -> Self {
        Self {
            inner: repository,
            cache,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryEvmStateRepository;
    use primitive_types::U256;

    struct DummyCache(Account);

    impl Cache<Address, Account> for DummyCache {
        fn read(&mut self, _key: &Address) -> Option<Account> {
            Some(self.0.clone())
        }

        fn write(&mut self, _key: Address, _value: Account) {}
    }

    struct EmptyCache(Option<Account>);

    impl Cache<Address, Account> for EmptyCache {
        fn read(&mut self, _key: &Address) -> Option<Account> {
            self.0.clone()
        }

        fn write(&mut self, _key: Address, value: Account) {
            self.0.replace(value);
        }
    }

    struct NoopEvmRepository;

    impl EvmStateRepository for NoopEvmRepository {
        fn get(&mut self, _address: &Address) -> Option<Account> {
            None
        }

        fn replace(&mut self, _address: Address, _account: Account) {}
    }

    #[test]
    fn test_account_is_primarily_taken_from_cache() {
        let expected_account = Account::new(4, U256::zero(), U256::zero(), U256::zero());
        let repository = NoopEvmRepository;
        let cache = DummyCache(expected_account.clone());
        let mut repository = CachedEvmStateRepository::new(repository, cache);

        let actual_account = repository.get(&[0u8; 20]);

        assert!(actual_account.is_some(), "Account not hit in cache");

        let actual_account = actual_account.unwrap();

        assert_eq!(expected_account, actual_account);
    }

    #[test]
    fn test_account_is_loaded_from_repository_when_cache_misses() {
        let expected_account = Account::new(4, U256::zero(), U256::zero(), U256::zero());
        let mut repository = InMemoryEvmStateRepository::new();
        repository.replace([0u8; 20], expected_account.clone());
        let cache = EmptyCache(None);
        let mut repository = CachedEvmStateRepository::new(repository, cache);

        let actual_account = repository.get(&[0u8; 20]);

        assert!(
            actual_account.is_some(),
            "Account not found using repository"
        );

        let actual_account = actual_account.unwrap();

        assert_eq!(expected_account, actual_account);
    }
}
