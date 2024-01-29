use dharitri_wasm::{api::VMApi, dharitri_codec::TryStaticCast};

use crate::DebugApi;

impl TryStaticCast for DebugApi {}

impl VMApi for DebugApi {}
