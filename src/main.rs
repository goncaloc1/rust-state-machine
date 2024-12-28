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

/// This test checks the following:
/// - That `alice` cannot transfer funds she does not have.
/// - That `alice` can successfully transfer funds to `bob`.
/// - That the balance of `alice` and `bob` is correctly updated.
#[test]
fn transfer_balance() {
    let mut balances = balances::Pallet::new();

    assert_eq!(
        balances.transfer("alice".to_string(), "bob".to_string(), 10),
        Err("error subtracting balance")
    );

    balances.set_balance(&"alice".to_string(), 15);

    assert_eq!(
        balances.transfer("alice".to_string(), "bob".to_string(), 10),
        Ok(())
    );

    assert_eq!(balances.balance(&"alice".to_string()), 5);
    assert_eq!(balances.balance(&"bob".to_string()), 10);
}
