use dharitri_wasm::types::{BoxedBytes, TokenIdentifier};
use dharitri_wasm_debug::{check_managed_top_decode, check_managed_top_encode_decode, DebugApi};

#[test]
fn test_moax() {
    let api = DebugApi::dummy();
    assert!(TokenIdentifier::moax(api).is_moax());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        TokenIdentifier::moax(api.clone()),
        TokenIdentifier::<DebugApi>::MOAX_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &TokenIdentifier::<DebugApi>::MOAX_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api.clone(),
        vec![TokenIdentifier::moax(api.clone())],
        expected.as_slice(),
    );

    // also allowed
    assert_eq!(
        TokenIdentifier::moax(api.clone()),
        check_managed_top_decode::<TokenIdentifier<DebugApi>>(api.clone(), &[])
    );
    assert_eq!(
        vec![TokenIdentifier::moax(api.clone())],
        check_managed_top_decode::<Vec<TokenIdentifier<DebugApi>>>(api, &[0, 0, 0, 0])
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dct_identifier() {
    let api = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::from_dct_bytes(api.clone(), &b"ALC-6258d2"[..]).is_valid_dct_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::from_dct_bytes(api.clone(), &b"ALC123-6258d2"[..]).is_valid_dct_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::from_dct_bytes(api.clone(), &b"12345-6258d2"[..]).is_valid_dct_identifier());

    // missing dash
    assert!(!TokenIdentifier::from_dct_bytes(api.clone(), &b"ALC6258d2"[..]).is_valid_dct_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::from_dct_bytes(api.clone(), &b"AL-C6258d2"[..]).is_valid_dct_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::from_dct_bytes(api.clone(), &b"alc-6258d2"[..]).is_valid_dct_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::from_dct_bytes(api.clone(), &b"ALC-6258D2"[..]).is_valid_dct_identifier());

    // too many random chars
    assert!(!TokenIdentifier::from_dct_bytes(api.clone(), &b"ALC-6258d2ff"[..]).is_valid_dct_identifier());

    // ticker too short
    assert!(!TokenIdentifier::from_dct_bytes(api.clone(), &b"AL-6258d2"[..]).is_valid_dct_identifier());

    // ticker too long
    assert!(!TokenIdentifier::from_dct_bytes(api, &b"ALCCCCCCCCC-6258d2"[..]).is_valid_dct_identifier());
}
