mod cached;
mod concurrent_in_memory;
mod in_memory;

pub use cached::*;
pub use concurrent_in_memory::*;
pub use in_memory::*;

use primitive_types::U256;

/// An Ethereum [address] uniquely identifies [`Account`].
///
/// [address]: https://ethereum.org/en/glossary/#address
pub type Address = [u8; 20];

/// An Ethereum [account] is an entity with an ether (ETH) balance that can send transactions.
///
/// It is a part of the EVM state and can be user-controlled or deployed as smart contracts.
///
/// [account]: https://ethereum.org/en/developers/docs/accounts/
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    nonce: u64,
    balance: U256,
    code_hash: U256,
    storage_root: U256,
}

impl Account {
    pub fn new(nonce: u64, balance: U256, code_hash: U256, storage_root: U256) -> Self {
        Self {
            nonce,
            balance,
            code_hash,
            storage_root,
        }
    }
}

/// A trait for objects capable of accessing [EVM state].
///
/// [EVM state]: https://ethereum.org/en/developers/docs/evm/#state
pub trait EvmStateRepository {
    fn get(&mut self, address: &Address) -> Option<Account>;
    fn replace(&mut self, address: Address, account: Account);
}
