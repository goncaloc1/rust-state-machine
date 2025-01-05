use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    /// Get the owner (if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    /// Create a new claim on behalf of the `caller`.
    /// This function will return an error if someone already has claimed that content.
    pub fn create_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content,
    ) -> crate::support::DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err(&"this content is already claimed");
        }

        self.claims.insert(claim, caller);

        Ok(())
    }

    /// Revoke an existing claim on some content.
    /// This function should only succeed if the caller is the owner of an existing claim.
    /// It will return an error if the claim does not exist, or if the caller is not the owner.
    pub fn revoke_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content,
    ) -> crate::support::DispatchResult {
        let owner = self
            .get_claim(&claim)
            .ok_or("revoking claim but it does not exist")?;

        if *owner != caller {
            return Err(&"revoking claim but caller does not match owner");
        }

        self.claims.remove(&claim);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut proof_of_existence = super::Pallet::<TestConfig>::new();

        let content_1 = "content 1";
        let alice = "alice";
        let bob = "bob";

        assert_eq!(proof_of_existence.get_claim(&content_1), None);

        assert_eq!(
            proof_of_existence.revoke_claim(alice, content_1),
            Err("revoking claim but it does not exist")
        );

        assert_eq!(proof_of_existence.create_claim(alice, content_1), Ok(()));

        assert_eq!(proof_of_existence.get_claim(&content_1), Some(&alice));

        assert_eq!(
            proof_of_existence.revoke_claim(bob, content_1),
            Err("revoking claim but caller does not match owner")
        );

        assert_eq!(proof_of_existence.revoke_claim(alice, content_1), Ok(()));

        assert_eq!(proof_of_existence.get_claim(&content_1), None);
    }
}
