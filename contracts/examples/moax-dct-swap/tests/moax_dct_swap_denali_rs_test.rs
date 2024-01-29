use dharitri_wasm::*;
use dharitri_wasm_debug::*;

#[allow(unused)]
fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/moax-dct-swap.wasm",
        Box::new(|context| Box::new(moax_dct_swap::contract_obj(context))),
    );
    blockchain
}

#[test]
fn unwrap_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/unwrap_moax.scen.json", world());
}

#[test]
fn wrap_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/wrap_moax.scen.json", world());
}
