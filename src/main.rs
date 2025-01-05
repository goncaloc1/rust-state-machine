use support::Dispatch;

mod balances;
mod proof_of_existence;
mod support;
mod system;

mod types {
    pub type AccountId = String;
    pub type BlockNumber = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u128;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

fn main() {
    // Create a new instance of the Runtime.
    // It will instantiate with it all the modules it uses.
    let mut runtime = Runtime::new();
    let alice = String::from("alice");
    let bob = String::from("bob");
    let charlie = String::from("charlie");

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header {
            block_number: runtime.system.block_number() + 1,
        },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: charlie.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");

    let block_2 = types::Block {
        header: support::Header {
            block_number: runtime.system.block_number() + 1,
        },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "claim content",
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "claim content",
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "claim content",
                }),
            },
            support::Extrinsic {
                caller: charlie.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "charlie claim content",
                }),
            },
        ],
    };

    runtime.execute_block(block_2).expect("invalid block");

    // Print the debug format of runtime state
    print!("{:#?}", runtime)
}
