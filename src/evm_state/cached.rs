use crate::cache::Cache;
use crate::evm_state::{Account, Address, EvmStateRepository};

pub struct CachedEvmStateRepository<InnerRepository: EvmStateRepository, C: Cache<Address, Account>>
{
    cache: C,
    inner: InnerRepository,
}

impl<InnerRepository: EvmStateRepository, C: Cache<Address, Account>> EvmStateRepository
    for CachedEvmStateRepository<InnerRepository, C>
{
    fn get(&mut self, address: &Address) -> Option<&Account> {
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
