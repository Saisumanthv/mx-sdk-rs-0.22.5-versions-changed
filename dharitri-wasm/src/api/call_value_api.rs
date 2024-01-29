use super::{ErrorApi, ManagedTypeApi};
use crate::{
    err_msg,
    types::{BigUint, DctTokenPayment, DctTokenType, ManagedVec, TokenIdentifier},
};

pub trait CallValueApi: ManagedTypeApi + ErrorApi + Sized {
    fn check_not_payable(&self);

    /// Retrieves the MOAX call value from the VM.
    /// Will return 0 in case of an DCT transfer (cannot have both MOAX and DCT transfer simultaneously).
    fn moax_value(&self) -> BigUint<Self>;

    /// Retrieves the DCT call value from the VM.
    /// Will return 0 in case of an MOAX transfer (cannot have both MOAX and DCT transfer simultaneously).
    fn dct_value(&self) -> BigUint<Self>;

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for MOAX,
    /// but the MOAX TokenIdentifier is serialized as `MOAX`.
    fn token(&self) -> TokenIdentifier<Self>;

    /// Returns the nonce of the received DCT token.
    /// Will return 0 in case of MOAX or fungible DCT transfer.
    fn dct_token_nonce(&self) -> u64;

    /// Returns the DCT token type.
    /// Will return "Fungible" for MOAX.
    fn dct_token_type(&self) -> DctTokenType;

    /// Will return the MOAX call value,
    /// but also fail with an error if DCT is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_moax(&self) -> BigUint<Self> {
        if !self.token().is_moax() {
            self.signal_error(err_msg::NON_PAYABLE_FUNC_DCT);
        }
        self.moax_value()
    }

    /// Will return the DCT call value,
    /// but also fail with an error if MOAX or the wrong DCT token is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_dct(&self, token: &[u8]) -> BigUint<Self> {
        if self.token().as_managed_buffer() != token {
            self.signal_error(err_msg::BAD_TOKEN_PROVIDED);
        }
        self.dct_value()
    }

    /// Returns both the call value (either MOAX or DCT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    fn payment_token_pair(&self) -> (BigUint<Self>, TokenIdentifier<Self>) {
        let token = self.token();
        if token.is_moax() {
            (self.moax_value(), token)
        } else {
            (self.dct_value(), token)
        }
    }

    fn dct_num_transfers(&self) -> usize;

    fn dct_value_by_index(&self, index: usize) -> BigUint<Self>;

    fn token_by_index(&self, index: usize) -> TokenIdentifier<Self>;

    fn dct_token_nonce_by_index(&self, index: usize) -> u64;

    fn dct_token_type_by_index(&self, index: usize) -> DctTokenType;

    fn get_all_dct_transfers(&self) -> ManagedVec<Self, DctTokenPayment<Self>> {
        let num_transfers = self.dct_num_transfers();
        let mut transfers = ManagedVec::new(self.clone());

        for i in 0..num_transfers {
            let token_type = self.dct_token_type_by_index(i);
            let token_identifier = self.token_by_index(i);
            let token_nonce = self.dct_token_nonce_by_index(i);
            let amount = self.dct_value_by_index(i);

            transfers.push(DctTokenPayment::<Self> {
                token_type,
                token_identifier,
                token_nonce,
                amount,
            });
        }

        transfers
    }
}
