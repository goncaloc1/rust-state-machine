mod balances;

fn main() {
    println!("Hello, world!");
}

#[test]
fn init_balances() {
  let mut balances = balances::Pallet::new();

  assert_eq!(balances.balance(&"alice".to_string()), 0);

  balances.set_balance(&"alice".to_string(), 100);

  assert_eq!(balances.balance(&"alice".to_string()), 100);
  assert_eq!(balances.balance(&"bob".to_string()), 0);
}
