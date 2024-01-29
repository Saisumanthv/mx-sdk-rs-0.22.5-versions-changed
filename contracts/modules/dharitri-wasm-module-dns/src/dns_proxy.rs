dharitri_wasm::imports!();

#[dharitri_wasm::proxy]
pub trait Dns {
    #[payable("MOAX")]
    #[endpoint]
    fn register(&self, name: BoxedBytes, #[payment] payment: BigUint);
}
