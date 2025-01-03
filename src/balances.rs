use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/// The configuration trait for the Balances Module.
/// Contains the basic types needed for handling balances.
pub trait Config: crate::system::Config {
    /// A type which can represent the balance of an account.
    /// Usually this is a large unsigned integer.
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances module
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        // dereferencing with `*` as result will be &value or &0
        // note, get will return Option<&V> hence the dereferencing
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("error subtracting balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("error adding balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);

        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    /// This test checks the following:
    /// - That `alice` cannot transfer funds she does not have.
    /// - That `alice` can successfully transfer funds to `bob`.
    /// - That the balance of `alice` and `bob` is correctly updated.
    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();
        let alice = String::from("alice");
        let bob = String::from("bob");

        assert_eq!(
            balances.transfer(alice.clone(), bob.clone(), 10),
            Err("error subtracting balance")
        );

        balances.set_balance(&alice, 15);

        assert_eq!(balances.transfer(alice.clone(), bob.clone(), 10), Ok(()));

        assert_eq!(balances.balance(&alice), 5);
        assert_eq!(balances.balance(&bob), 10);
    }
}
