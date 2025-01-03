use support::Dispatch;

mod balances;
mod support;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    BalancesTransfer {
        to: types::AccountId,
        amount: types::Balance,
    },
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u128;
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
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
            return Err("Block numbers do not match");
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
            RuntimeCall::BalancesTransfer { to, amount } => {
                self.balances.transfer(caller, to, amount)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = String::from("alice");
    let bob = String::from("bob");
    let charlie = String::from("charlie");

    runtime.balances.set_balance(&alice, 100);

    // start emulating a block
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    runtime.system.inc_nonce(&alice);

    let _ = runtime
        .balances
        .transfer(alice.clone(), bob, 30)
        .map_err(|e| eprintln!("{}", e));

    // second transaction
    runtime.system.inc_nonce(&alice);

    let _ = runtime
        .balances
        .transfer(alice, charlie, 20)
        .map_err(|e| eprintln!("{}", e));

    print!("{:#?}", runtime)
}
