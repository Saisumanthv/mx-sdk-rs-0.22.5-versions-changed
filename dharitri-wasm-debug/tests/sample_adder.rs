// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

use dharitri_wasm::{
    contract_base::{ContractBase, ProxyObjBase},
    types::{BigInt, ManagedAddress},
};

use crate::module_1::VersionModule;

mod module_1 {
    dharitri_wasm::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: dharitri_wasm::contract_base::ContractBase + Sized {
        fn version(&self) -> BigInt<Self::Api>;

        fn some_async(&self) -> AsyncCall<Self::Api>;

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: dharitri_wasm::contract_base::ContractBase {}

    impl<C> VersionModule for C
    where
        C: AutoImpl,
    {
        fn version(&self) -> BigInt<Self::Api> {
            BigInt::from_i64(self.type_manager(), 100)
        }

        fn some_async(&self) -> AsyncCall<Self::Api> {
            panic!("wooo")
        }

        fn callback(&self) {}
    }

    pub trait EndpointWrappers: VersionModule + dharitri_wasm::contract_base::ContractBase {
        #[inline]
        fn call_version(&self) {
            dharitri_wasm::api::CallValueApi::check_not_payable(&self.raw_vm_api());
            let result = self.version();
            dharitri_wasm::io::EndpointResult::finish(&result, self.raw_vm_api())
        }

        fn call_some_async(&self) {
            let result = self.some_async();
            dharitri_wasm::io::EndpointResult::finish(&result, self.raw_vm_api())
        }

        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self.callback();
                    return true;
                },
                b"version" => {
                    self.call_version();
                    true
                },
                _other => false,
            } {
                return true;
            }
            false
        }
    }
    pub struct AbiProvider {}

    impl dharitri_wasm::contract_base::ContractAbiProvider for AbiProvider {
        type Api = dharitri_wasm::api::uncallable::UncallableApi;

        fn abi() -> dharitri_wasm::abi::ContractAbi {
            dharitri_wasm::abi::ContractAbi::default()
        }
    }

    pub trait ProxyTrait: dharitri_wasm::contract_base::ProxyObjBase + Sized {
        fn version(
            self,
        ) -> ContractCall<Self::Api, <BigInt<Self::Api> as dharitri_wasm::io::EndpointResult>::DecodeAs>
        {
            let (___api___, ___address___) = self.into_fields();
            let mut ___contract_call___ = dharitri_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                &b"version"[..],
                ManagedVec::new(___api___.clone()),
            );
            ___contract_call___
        }
    }
}

mod sample_adder {
    dharitri_wasm::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait Adder:
        super::module_1::VersionModule + dharitri_wasm::contract_base::ContractBase + Sized
    {
        fn init(&self, initial_value: &BigInt<Self::Api>) {
            self.set_sum(initial_value);
        }
        fn add(&self, value: BigInt<Self::Api>) -> SCResult<()> {
            let mut sum = self.get_sum();
            sum.add_assign(value);
            self.set_sum(&sum);
            Ok(())
        }
        fn get_sum(&self) -> BigInt<Self::Api>;
        fn set_sum(&self, sum: &BigInt<Self::Api>);
        fn add_version(&self) -> SCResult<()> {
            self.add(self.version())
        }
        fn callback(&self);
        fn callbacks(&self) -> self::CallbackProxyObj<Self::Api>;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: dharitri_wasm::contract_base::ContractBase {}

    // impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        fn get_sum(&self) -> BigInt<Self::Api> {
            let mut ___key___ =
                dharitri_wasm::storage::StorageKey::<Self::Api>::new(self.raw_vm_api(), &b"sum"[..]);
            dharitri_wasm::storage_get(self.raw_vm_api(), &___key___)
        }
        fn set_sum(&self, sum: &BigInt<Self::Api>) {
            let mut ___key___ =
                dharitri_wasm::storage::StorageKey::<Self::Api>::new(self.raw_vm_api(), &b"sum"[..]);
            dharitri_wasm::storage_set(self.raw_vm_api(), &___key___, &sum);
        }
        fn callback(&self) {}
        fn callbacks(&self) -> self::CallbackProxyObj<Self::Api> {
            <self::CallbackProxyObj::<Self::Api> as dharitri_wasm::contract_base::CallbackProxyObjBase>::new_cb_proxy_obj(self.raw_vm_api())
        }
    }

    pub trait EndpointWrappers:
        Adder + dharitri_wasm::contract_base::ContractBase + super::module_1::EndpointWrappers
    {
        #[inline]
        fn call_get_sum(&self) {
            dharitri_wasm::api::CallValueApi::check_not_payable(&self.raw_vm_api());
            dharitri_wasm::api::EndpointArgumentApi::check_num_arguments(&self.raw_vm_api(), 0i32);
            let result = self.get_sum();
            dharitri_wasm::io::EndpointResult::finish(&result, self.raw_vm_api());
        }
        #[inline]
        fn call_init(&self) {
            dharitri_wasm::api::CallValueApi::check_not_payable(&self.raw_vm_api());
            dharitri_wasm::api::EndpointArgumentApi::check_num_arguments(&self.raw_vm_api(), 1i32);
            let initial_value = dharitri_wasm::load_single_arg::<Self::Api, BigInt<Self::Api>>(
                self.raw_vm_api(),
                0i32,
                ArgId::from(&b"initial_value"[..]),
            );
            self.init(&initial_value);
        }
        #[inline]
        fn call_add(&self) {
            dharitri_wasm::api::CallValueApi::check_not_payable(&self.raw_vm_api());
            dharitri_wasm::api::EndpointArgumentApi::check_num_arguments(&self.raw_vm_api(), 1i32);
            let value = dharitri_wasm::load_single_arg::<Self::Api, BigInt<Self::Api>>(
                self.raw_vm_api(),
                0i32,
                ArgId::from(&b"value"[..]),
            );
            let result = self.add(value);
            dharitri_wasm::io::EndpointResult::finish(&result, self.raw_vm_api());
        }

        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    Adder::callback(self);
                    return true;
                },
                [103u8, 101u8, 116u8, 83u8, 117u8, 109u8] => {
                    self.call_get_sum();
                    true
                },
                [105u8, 110u8, 105u8, 116u8] => {
                    self.call_init();
                    true
                },
                [97u8, 100u8, 100u8] => {
                    self.call_add();
                    true
                },
                _other => false,
            } {
                return true;
            }
            if super::module_1::EndpointWrappers::call(self, fn_name) {
                return true;
            }
            false
        }
    }

    pub trait ProxyTrait:
        dharitri_wasm::contract_base::ProxyObjBase + super::module_1::ProxyTrait
    {
        fn get_sum(
            self,
        ) -> dharitri_wasm::types::ContractCall<
            Self::Api,
            <BigInt<Self::Api> as dharitri_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___) = self.into_fields();
            let mut ___contract_call___ = dharitri_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                &b"get_sum"[..],
                ManagedVec::new(___api___.clone()),
            );
            ___contract_call___
        }
        fn add(
            self,
            amount: &BigInt<Self::Api>,
        ) -> ContractCall<Self::Api, <SCResult<()> as dharitri_wasm::io::EndpointResult>::DecodeAs>
        {
            let (___api___, ___address___) = self.into_fields();
            let mut ___contract_call___ = dharitri_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                &b"add"[..],
                ManagedVec::new(___api___.clone()),
            );
            ___contract_call___.push_endpoint_arg(amount);
            ___contract_call___
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub struct ContractObj<A>
    where
        A: dharitri_wasm::api::VMApi + Clone + 'static,
    {
        api: A,
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    impl<A> dharitri_wasm::contract_base::ContractBase for ContractObj<A>
    where
        A: dharitri_wasm::api::VMApi + Clone + 'static,
    {
        type Api = A;

        fn raw_vm_api(&self) -> Self::Api {
            self.api.clone()
        }
    }

    impl<A> super::module_1::AutoImpl for ContractObj<A> where
        A: dharitri_wasm::api::VMApi + Clone + 'static
    {
    }

    impl<A> AutoImpl for ContractObj<A> where A: dharitri_wasm::api::VMApi + Clone + 'static {}

    impl<A> super::module_1::EndpointWrappers for ContractObj<A> where
        A: dharitri_wasm::api::VMApi + Clone + 'static
    {
    }

    impl<A> EndpointWrappers for ContractObj<A> where A: dharitri_wasm::api::VMApi + Clone + 'static {}

    impl<A> dharitri_wasm::contract_base::CallableContract<A> for ContractObj<A>
    where
        A: dharitri_wasm::api::VMApi + Clone + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }

    pub struct AbiProvider {}

    impl dharitri_wasm::contract_base::ContractAbiProvider for AbiProvider {
        type Api = dharitri_wasm::api::uncallable::UncallableApi;

        fn abi() -> dharitri_wasm::abi::ContractAbi {
            dharitri_wasm::abi::ContractAbi::default()
        }
    }

    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A: dharitri_wasm::api::VMApi + Clone + 'static,
    {
        ContractObj { api }
    }

    pub struct Proxy<A>
    where
        A: dharitri_wasm::api::VMApi + 'static,
    {
        pub api: A,
        pub address: dharitri_wasm::types::ManagedAddress<A>,
    }

    impl<A> dharitri_wasm::contract_base::ProxyObjBase for Proxy<A>
    where
        A: dharitri_wasm::api::VMApi + 'static,
    {
        type Api = A;

        fn new_proxy_obj(api: A) -> Self {
            let zero_address = ManagedAddress::zero(api.clone());
            Proxy {
                api,
                address: zero_address,
            }
        }

        fn contract(mut self, address: ManagedAddress<Self::Api>) -> Self {
            self.address = address;
            self
        }

        #[inline]
        fn into_fields(self) -> (Self::Api, ManagedAddress<Self::Api>) {
            (self.api, self.address)
        }
    }

    impl<A> super::module_1::ProxyTrait for Proxy<A> where A: dharitri_wasm::api::VMApi {}

    impl<A> ProxyTrait for Proxy<A> where A: dharitri_wasm::api::VMApi {}

    pub struct CallbackProxyObj<A>
    where
        A: dharitri_wasm::api::VMApi + 'static,
    {
        pub api: A,
    }

    impl<A> dharitri_wasm::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
    where
        A: dharitri_wasm::api::VMApi + 'static,
    {
        type Api = A;

        fn new_cb_proxy_obj(api: A) -> Self {
            CallbackProxyObj { api }
        }
        fn cb_call_api(self) -> Self::Api {
            self.api.clone()
        }
    }

    pub trait CallbackProxy: dharitri_wasm::contract_base::CallbackProxyObjBase + Sized {
        fn my_callback(self, caller: &Address) -> dharitri_wasm::types::CallbackClosure<Self::Api> {
            let mut ___callback_call___ =
                dharitri_wasm::types::new_callback_call(self.cb_call_api(), &b"my_callback"[..]);
            ___callback_call___.push_endpoint_arg(caller);
            ___callback_call___
        }
    }
    impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: dharitri_wasm::api::VMApi + 'static {}
}

#[test]
fn test_add() {
    use dharitri_wasm_debug::DebugApi;
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let tx_context = DebugApi::dummy();

    let adder = sample_adder::contract_obj(tx_context.clone());

    adder.init(&BigInt::from_i64(adder.type_manager(), 5));
    assert_eq!(BigInt::from_i64(adder.type_manager(), 5), adder.get_sum());

    let _ = adder.add(BigInt::from_i64(adder.type_manager(), 7));
    assert_eq!(BigInt::from_i64(adder.type_manager(), 12), adder.get_sum());

    let _ = adder.add(BigInt::from_i64(adder.type_manager(), -1));
    assert_eq!(BigInt::from_i64(adder.type_manager(), 11), adder.get_sum());

    assert_eq!(BigInt::from_i64(adder.type_manager(), 100), adder.version());

    let _ = adder.add_version();
    assert_eq!(BigInt::from_i64(adder.type_manager(), 111), adder.get_sum());

    assert!(!adder.call(b"invalid_endpoint"));

    assert!(adder.call(b"version"));

    let own_proxy = sample_adder::Proxy::new_proxy_obj(tx_context.clone())
        .contract(ManagedAddress::zero(tx_context));
    let _ = own_proxy.get_sum();

    let _ = dharitri_wasm_debug::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

fn world() -> dharitri_wasm_debug::BlockchainMock {
    let mut blockchain = dharitri_wasm_debug::BlockchainMock::new();
    blockchain.register_contract(
        "file:../contracts/examples/adder/output/adder.wasm",
        Box::new(|context| Box::new(sample_adder::contract_obj(context))),
    );
    blockchain
}

#[test]
fn test_denali() {
    dharitri_wasm_debug::denali_rs(
        "../contracts/examples/adder/denali/adder.scen.json",
        world(),
    );
}
