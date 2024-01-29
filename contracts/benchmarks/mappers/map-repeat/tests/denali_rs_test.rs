use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/map-repeat");

    blockchain.register_contract(
        "file:output/map-repeat.wasm",
        Box::new(|context| Box::new(map_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn map_repeat_struct_rs() {
    dharitri_wasm_debug::denali_rs("denali/map_repeat_struct.scen.json", world());
}

#[test]
fn map_repeat_rs() {
    dharitri_wasm_debug::denali_rs("denali/map_repeat.scen.json", world());
}
