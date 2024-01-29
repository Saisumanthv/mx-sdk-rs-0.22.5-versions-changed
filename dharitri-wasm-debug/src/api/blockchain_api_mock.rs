use crate::{
    world_mock::{is_smart_contract_address, DctInstance},
    DebugApi,
};
use dharitri_wasm::types::{
    Address, BigUint, DctTokenData, DctTokenType, ManagedAddress, ManagedBuffer, ManagedVec,
    TokenIdentifier, H256,
};

impl dharitri_wasm::api::BlockchainApi for DebugApi {
    fn get_sc_address_legacy(&self) -> Address {
        self.input_ref().to.clone()
    }

    fn get_owner_address_legacy(&self) -> Address {
        self.with_contract_account(|account| {
            account
                .contract_owner
                .clone()
                .unwrap_or_else(|| panic!("contract owner address not set"))
        })
    }

    fn get_shard_of_address_legacy(&self, _address: &Address) -> u32 {
        panic!("get_shard_of_address not implemented")
    }

    fn is_smart_contract_legacy(&self, address: &Address) -> bool {
        is_smart_contract_address(address)
    }

    fn get_caller_legacy(&self) -> Address {
        self.input_ref().from.clone()
    }

    fn get_balance_legacy(&self, address: &Address) -> BigUint<Self> {
        assert!(
            address == &self.get_sc_address_legacy(),
            "get balance not yet implemented for accounts other than the contract itself"
        );
        let moax_balance = self.with_contract_account(|account| account.moax_balance.clone());
        self.insert_new_big_uint(moax_balance)
    }

    fn get_state_root_hash_legacy(&self) -> H256 {
        panic!("get_state_root_hash_legacy not yet implemented")
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        self.input_ref().tx_hash.clone()
    }

    fn get_gas_left(&self) -> u64 {
        self.input_ref().gas_limit
    }

    fn get_block_timestamp(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_timestamp
    }

    fn get_block_nonce(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_nonce
    }

    fn get_block_round(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_round
    }

    fn get_block_epoch(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_epoch
    }

    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        self.blockchain_ref()
            .current_block_info
            .block_random_seed
            .clone()
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_timestamp
    }

    fn get_prev_block_nonce(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_nonce
    }

    fn get_prev_block_round(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_round
    }

    fn get_prev_block_epoch(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_epoch
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        self.blockchain_ref()
            .previous_block_info
            .block_random_seed
            .clone()
    }

    fn get_current_dct_nft_nonce(&self, address: &Address, token: &TokenIdentifier<Self>) -> u64 {
        assert!(
            address == &self.get_sc_address_legacy(),
            "get_current_dct_nft_nonce not yet implemented for accounts other than the contract itself"
        );

        self.with_contract_account(|account| {
            account
                .dct
                .get_by_identifier_or_default(token.to_dct_identifier().as_slice())
                .last_nonce
        })
    }

    fn get_dct_balance(
        &self,
        address: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        nonce: u64,
    ) -> BigUint<Self> {
        assert!(
            address == &self.get_sc_address(),
            "get_dct_balance not yet implemented for accounts other than the contract itself"
        );

        let dct_balance = self.with_contract_account(|account| {
            account
                .dct
                .get_dct_balance(token.to_dct_identifier().as_slice(), nonce)
        });
        self.insert_new_big_uint(dct_balance)
    }

    fn get_dct_token_data(
        &self,
        address: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        nonce: u64,
    ) -> DctTokenData<Self> {
        self.blockchain_cache()
            .with_account(&address.to_address(), |account| {
                let instance = account
                    .dct
                    .get_by_identifier(token.to_dct_identifier().as_slice())
                    .unwrap()
                    .instances
                    .get_by_nonce(nonce)
                    .unwrap();

                self.dct_token_data_from_instance(nonce, instance)
            })
    }
}

impl DebugApi {
    fn dct_token_data_from_instance(
        &self,
        nonce: u64,
        instance: &DctInstance,
    ) -> DctTokenData<Self> {
        let creator = if let Some(creator) = &instance.metadata.creator {
            ManagedAddress::from_address(self.clone(), creator)
        } else {
            ManagedAddress::zero(self.clone())
        };

        let mut uris = ManagedVec::new(self.clone());
        if let Some(uri) = &instance.metadata.uri {
            uris.push(ManagedBuffer::new_from_bytes(self.clone(), uri.as_slice()));
        }

        DctTokenData {
            token_type: DctTokenType::based_on_token_nonce(nonce),
            amount: self.insert_new_big_uint(instance.balance.clone()),
            frozen: false,
            hash: self
                .insert_new_managed_buffer(instance.metadata.hash.clone().unwrap_or_default()),
            name: self.insert_new_managed_buffer(instance.metadata.name.clone()),
            attributes: self.insert_new_managed_buffer(instance.metadata.attributes.clone()),
            creator,
            royalties: self
                .insert_new_big_uint(num_bigint::BigUint::from(instance.metadata.royalties)),
            uris,
        }
    }
}
