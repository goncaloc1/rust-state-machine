use std::collections::BTreeMap;

use num::{One, Zero};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountId, Nonce> {
    /// The current block number.
    block_number: BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<BlockNumber, AccountId, Nonce> Pallet<BlockNumber, AccountId, Nonce>
where
    BlockNumber: Zero + One + Copy,
    AccountId: Ord + Clone,
    Nonce: Zero + One + Copy,
{
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number + BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountId) {
        let current_nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        let new_nonce = current_nonce + Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test {
    use crate::types;

    /// Checks the following:
    /// - Increment the current block number.
    /// - Increment the nonce of `alice`.
    /// - Check the block number is what we expect.
    /// - Check the nonce of `alice` is what we expect.
    #[test]
    fn init_system() {
        let mut system = super::Pallet::<types::BlockNumber, types::AccountId, types::Nonce>::new();
        let alice = String::from("alice");
        let bob = String::from("bob");

        assert_eq!(system.block_number, 0);

        system.inc_block_number();
        system.inc_nonce(&alice);

        assert_eq!(system.block_number, 1);
        assert_eq!(system.nonce.get(&alice), Some(&1));
        assert_eq!(system.nonce.get(&bob), None);
    }
}
