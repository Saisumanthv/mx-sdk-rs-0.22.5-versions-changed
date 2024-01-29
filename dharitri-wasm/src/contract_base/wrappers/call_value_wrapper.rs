use crate::{
    api::{CallValueApi, ErrorApi, ManagedTypeApi},
    types::{BigUint, DctTokenPayment, DctTokenType, ManagedVec, TokenIdentifier},
};

pub struct CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub(crate) api: A,
}

impl<A> CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub(crate) fn new(api: A) -> Self {
        CallValueWrapper { api }
    }

    /// Retrieves the MOAX call value from the VM.
    /// Will return 0 in case of an DCT transfer (cannot have both MOAX and DCT transfer simultaneously).
    pub fn moax_value(&self) -> BigUint<A> {
        self.api.moax_value()
    }

    /// Returns all DCT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just MOAX.
    /// Fully managed underlying types, very efficient.
    pub fn all_dct_transfers(&self) -> ManagedVec<A, DctTokenPayment<A>> {
        self.api.get_all_dct_transfers()
    }

    /// Retrieves the DCT call value from the VM.
    /// Will return 0 in case of an MOAX transfer (cannot have both MOAX and DCT transfer simultaneously).
    /// Warning, not tested with multi transfer, use `all_dct_transfers` instead!
    pub fn dct_value(&self) -> BigUint<A> {
        self.api.dct_value()
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for MOAX,
    /// but the MOAX TokenIdentifier is serialized as `MOAX`.
    /// Warning, not tested with multi transfer, use `all_dct_transfers` instead!
    pub fn token(&self) -> TokenIdentifier<A> {
        self.api.token()
    }

    /// Returns the nonce of the received DCT token.
    /// Will return 0 in case of MOAX or fungible DCT transfer.
    /// Warning, not tested with multi transfer, use `all_dct_transfers` instead!
    pub fn dct_token_nonce(&self) -> u64 {
        self.api.dct_token_nonce()
    }

    /// Returns the DCT token type.
    /// Will return "Fungible" for MOAX.
    /// Warning, not tested with multi transfer, use `all_dct_transfers` instead!
    pub fn dct_token_type(&self) -> DctTokenType {
        self.api.dct_token_type()
    }

    /// Returns both the call value (either MOAX or DCT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    /// TODO: replace with multi transfer handling everywhere
    pub fn payment_token_pair(&self) -> (BigUint<A>, TokenIdentifier<A>) {
        self.api.payment_token_pair()
    }
}
