use crate::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedVecItem},
};

use super::{DctTokenType, TokenIdentifier};

use dharitri_codec::dharitri_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use crate as dharitri_wasm; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Debug)]
pub struct DctTokenPayment<M: ManagedTypeApi> {
    pub token_type: DctTokenType,
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> DctTokenPayment<M> {
    pub fn no_payment(api: M) -> Self {
        DctTokenPayment {
            token_type: DctTokenType::Invalid,
            token_identifier: TokenIdentifier::moax(api.clone()),
            token_nonce: 0,
            amount: BigUint::zero(api),
        }
    }

    pub fn new(token_identifier: TokenIdentifier<M>, token_nonce: u64, amount: BigUint<M>) -> Self {
        let token_type = if amount != 0 && token_identifier.is_dct() {
            if token_nonce == 0 {
                DctTokenType::Fungible
            } else if amount == 1u64 {
                DctTokenType::NonFungible
            } else {
                DctTokenType::SemiFungible
            }
        } else {
            DctTokenType::Invalid
        };

        DctTokenPayment {
            token_type,
            token_identifier,
            token_nonce,
            amount,
        }
    }
}

fn managed_vec_item_from_slice<M, T>(api: M, arr: &[u8], index: &mut usize) -> T
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    ManagedVecItem::<M>::from_byte_reader(api, |bytes| {
        let size = T::PAYLOAD_SIZE;
        bytes.copy_from_slice(&arr[*index..*index + size]);
        *index += size;
    })
}

fn managed_vec_item_to_slice<M, T>(arr: &mut [u8], index: &mut usize, item: &T)
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    ManagedVecItem::<M>::to_byte_writer(item, |bytes| {
        let size = T::PAYLOAD_SIZE;
        arr[*index..*index + size].copy_from_slice(bytes);
        *index += size;
    });
}

impl<M: ManagedTypeApi> ManagedVecItem<M> for DctTokenPayment<M> {
    const PAYLOAD_SIZE: usize = 16;
    const SKIPS_RESERIALIZATION: bool = false;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, mut reader: Reader) -> Self {
        let mut arr: [u8; 16] = [0u8; 16];
        reader(&mut arr[..]);
        let mut index = 0;

        let token_identifier = managed_vec_item_from_slice(api.clone(), &arr, &mut index);
        let token_nonce = managed_vec_item_from_slice(api.clone(), &arr, &mut index);
        let amount = managed_vec_item_from_slice(api, &arr, &mut index);

        let token_type = if token_nonce > 0 {
            DctTokenType::SemiFungible
        } else {
            DctTokenType::Fungible
        };

        DctTokenPayment {
            token_type,
            token_identifier,
            token_nonce,
            amount,
        }
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let mut arr: [u8; 16] = [0u8; 16];
        let mut index = 0;

        managed_vec_item_to_slice::<M, _>(&mut arr, &mut index, &self.token_identifier);
        managed_vec_item_to_slice::<M, _>(&mut arr, &mut index, &self.token_nonce);
        managed_vec_item_to_slice::<M, _>(&mut arr, &mut index, &self.amount);

        writer(&arr[..])
    }
}
