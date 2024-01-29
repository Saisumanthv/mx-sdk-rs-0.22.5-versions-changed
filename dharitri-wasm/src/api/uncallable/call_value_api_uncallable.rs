use crate::{
    api::CallValueApi,
    types::{BigUint, DctTokenType, TokenIdentifier},
};

use super::UncallableApi;

impl CallValueApi for UncallableApi {
    fn check_not_payable(&self) {
        unreachable!()
    }

    fn moax_value(&self) -> BigUint<Self> {
        unreachable!()
    }

    fn dct_value(&self) -> BigUint<Self> {
        unreachable!()
    }

    fn token(&self) -> TokenIdentifier<Self> {
        unreachable!()
    }

    fn dct_token_nonce(&self) -> u64 {
        unreachable!()
    }

    fn dct_token_type(&self) -> DctTokenType {
        unreachable!()
    }

    fn dct_num_transfers(&self) -> usize {
        unreachable!()
    }

    fn dct_value_by_index(&self, _index: usize) -> BigUint<Self> {
        unreachable!()
    }

    fn token_by_index(&self, _index: usize) -> TokenIdentifier<Self> {
        unreachable!()
    }

    fn dct_token_nonce_by_index(&self, _index: usize) -> u64 {
        unreachable!()
    }

    fn dct_token_type_by_index(&self, _index: usize) -> DctTokenType {
        unreachable!()
    }
}
