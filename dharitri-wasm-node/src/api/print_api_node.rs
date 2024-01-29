use crate::VmApiImpl;
use dharitri_wasm::{api::PrintApi, types::BigUint};

impl PrintApi for VmApiImpl {
    #[inline(always)]
    fn print_biguint(&self, _biguint: &BigUint<Self>) {}
}
