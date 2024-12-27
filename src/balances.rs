use std::collections::BTreeMap;

pub struct Pallet {
  balances: BTreeMap<String, u128>
}

impl Pallet {
  /// Create a new instance of the balances module
	pub fn new() -> Self {
		Self {
			balances: BTreeMap::new()
		}
	}

  /// Set the balance of an account `who` to some `amount`
  pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}

  /// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
  pub fn balance(&self, who: &String) -> u128 {
    // dereferencing with `*` as result will be &value or &0
    // note, get will return Option<&V> hence the dereferencing
    *self.balances.get(who).unwrap_or(&0)
  }
}