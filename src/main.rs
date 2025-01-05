use support::Dispatch;

mod balances;
mod proof_of_existence;
mod support;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
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

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        /* This is what we're doing:
            - Increment the system's block number.
            - Check that the block number of the incoming block matches the current block number,
              or return an error.
            - Iterate over the extrinsics in the block...
                - Increment the nonce of the caller.
                - Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
                - Handle errors from `dispatch` same as we did for individual calls: printing any
                  error and capturing the result.
                - You can extend the error message to include information like the block number and
                  extrinsic number.
        */

        self.system.inc_block_number();

        if self.system.block_number() != block.header.block_number {
            eprintln!(
            "Detailed error: Block numbers do not match: system block_number {}; block_number {}.",
            self.system.block_number(), block.header.block_number
        );

            return Err("Block numbers do not match.");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);

            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        // This match statement will allow us to correctly route `RuntimeCall`s
        // to the appropriate pallet level function.
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }

        Ok(())
    }
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
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::transfer {
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
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "claim content",
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
                    claim: "claim content",
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
                    claim: "claim content",
                }),
            },
            support::Extrinsic {
                caller: charlie.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "charlie claim content",
                }),
            },
        ],
    };

    runtime.execute_block(block_2).expect("invalid block");

    // Print the debug format of runtime state
    print!("{:#?}", runtime)
}
