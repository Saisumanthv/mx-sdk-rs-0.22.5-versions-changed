use dharitri_wasm::types::{ManagedBuffer, ManagedFrom, TokenIdentifier};
use dharitri_wasm_debug::*;

use basic_features::token_identifier_features::TokenIdentifierFeatures;

#[test]
fn test_token_identifier_moax() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.token_identifier_moax();
    assert_eq!(TokenIdentifier::moax(context), result);
}

/// This just tests the contract syntax.
/// For a complete suite of test cases, see `dharitri-wasm-debug/tests/managed_token_identifier_test.rs`.
#[test]
fn test_token_identifier_is_valid() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.token_identifier_is_valid_1(&b"ALC-6258d2"[..]);
    assert!(result);
    let result = bf.token_identifier_is_valid_1(&b"AL-C6258d2"[..]);
    assert!(!result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::managed_from(
        context.clone(),
        &b"12345-6258d2"[..],
    ));
    assert!(result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::managed_from(
        context.clone(),
        &b"ALCCCCCCCCC-6258d2"[..],
    ));
    assert!(!result);
}
