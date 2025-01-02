use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

/// The configuration trait for the System Pallet.
/// This controls the common types used throughout our state machine.
pub trait Config {
    /// A type which can identify an account in our state machine.
    /// On a real blockchain, you would want this to be a cryptographic public key.
    type AccountId: Ord + Clone;

    /// A type which can be used to represent the current block number.
    /// Usually a basic unsigned integer.
    type BlockNumber: Zero + One + AddAssign + Copy;

    /// A type which can be used to keep track of the number of transactions from each account.
    /// Usually a basic unsigned integer.
    type Nonce: Zero + One + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    block_number: T::BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let current_nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = current_nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    /// Checks the following:
    /// - Increment the current block number.
    /// - Increment the nonce of `alice`.
    /// - Check the block number is what we expect.
    /// - Check the nonce of `alice` is what we expect.
    #[test]
    fn init_system() {
        let mut system = super::Pallet::<TestConfig>::new();
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
