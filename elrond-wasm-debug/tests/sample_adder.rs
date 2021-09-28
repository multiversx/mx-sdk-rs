// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

use elrond_wasm::{
    contract_base::{ContractBase, ProxyObjBase},
    types::{BigInt, ManagedAddress},
};

use crate::module_1::VersionModule;

mod module_1 {
    elrond_wasm::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: elrond_wasm::contract_base::ContractBase + Sized {
        fn version(&self) -> BigInt<Self::Api>;

        fn some_async(&self) -> AsyncCall<Self::Api>;

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: elrond_wasm::contract_base::ContractBase {}

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

    pub trait EndpointWrappers: VersionModule + elrond_wasm::contract_base::ContractBase {
        #[inline]
        fn call_version(&self) {
            elrond_wasm::api::CallValueApi::check_not_payable(&self.raw_vm_api());
            let result = self.version();
            elrond_wasm::io::EndpointResult::finish(&result, self.raw_vm_api())
        }

        fn call_some_async(&self) {
            let result = self.some_async();
            elrond_wasm::io::EndpointResult::finish(&result, self.raw_vm_api())
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

    impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
        type Api = elrond_wasm::api::uncallable::UncallableApi;

        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { build_info : elrond_wasm :: abi :: BuildInfoAbi { contract_crate : elrond_wasm :: abi :: ContractCrateBuildAbi { name : "adder" , version : "0.0.0" , } , framework : elrond_wasm :: abi :: FrameworkBuildAbi :: create () , } , docs : & ["One of the simplest smart contracts possible," , "it holds a single variable in storage, which anyone can increment."] , name : "Adder" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "version",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_output::<BigInt<Self::Api>>(&[]);
            contract_abi.add_type_descriptions::<BigInt<Self::Api>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }

    pub trait ProxyTrait: elrond_wasm::contract_base::ProxyObjBase + Sized {
        fn version(
            self,
        ) -> ContractCall<Self::Api, <BigInt<Self::Api> as elrond_wasm::io::EndpointResult>::DecodeAs>
        {
            let (___api___, ___address___) = self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                &b"version"[..],
            );
            ___contract_call___
        }
    }
}

mod sample_adder {
    elrond_wasm::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait Adder:
        super::module_1::VersionModule + elrond_wasm::contract_base::ContractBase + Sized
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
    pub trait AutoImpl: elrond_wasm::contract_base::ContractBase {}

    // impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        fn get_sum(&self) -> BigInt<Self::Api> {
            let mut ___key___ =
                elrond_wasm::storage::StorageKey::<Self::Api>::new(self.raw_vm_api(), &b"sum"[..]);
            elrond_wasm::storage_get(self.raw_vm_api(), &___key___)
        }
        fn set_sum(&self, sum: &BigInt<Self::Api>) {
            let mut ___key___ =
                elrond_wasm::storage::StorageKey::<Self::Api>::new(self.raw_vm_api(), &b"sum"[..]);
            elrond_wasm::storage_set(self.raw_vm_api(), &___key___, &sum);
        }
        fn callback(&self) {}
        fn callbacks(&self) -> self::CallbackProxyObj<Self::Api> {
            <self::CallbackProxyObj::<Self::Api> as elrond_wasm::contract_base::CallbackProxyObjBase>::new_cb_proxy_obj(self.raw_vm_api())
        }
    }

    pub trait EndpointWrappers:
        Adder + elrond_wasm::contract_base::ContractBase + super::module_1::EndpointWrappers
    {
        #[inline]
        fn call_get_sum(&self) {
            elrond_wasm::api::CallValueApi::check_not_payable(&self.raw_vm_api());
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.raw_vm_api(), 0i32);
            let result = self.get_sum();
            elrond_wasm::io::EndpointResult::finish(&result, self.raw_vm_api());
        }
        #[inline]
        fn call_init(&self) {
            elrond_wasm::api::CallValueApi::check_not_payable(&self.raw_vm_api());
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.raw_vm_api(), 1i32);
            let initial_value = elrond_wasm::load_single_arg::<Self::Api, BigInt<Self::Api>>(
                self.raw_vm_api(),
                0i32,
                ArgId::from(&b"initial_value"[..]),
            );
            self.init(&initial_value);
        }
        #[inline]
        fn call_add(&self) {
            elrond_wasm::api::CallValueApi::check_not_payable(&self.raw_vm_api());
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.raw_vm_api(), 1i32);
            let value = elrond_wasm::load_single_arg::<Self::Api, BigInt<Self::Api>>(
                self.raw_vm_api(),
                0i32,
                ArgId::from(&b"value"[..]),
            );
            let result = self.add(value);
            elrond_wasm::io::EndpointResult::finish(&result, self.raw_vm_api());
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
        elrond_wasm::contract_base::ProxyObjBase + super::module_1::ProxyTrait
    {
        fn get_sum(
            self,
        ) -> elrond_wasm::types::ContractCall<
            Self::Api,
            <BigInt<Self::Api> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___) = self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                &b"get_sum"[..],
            );
            ___contract_call___
        }
        fn add(
            self,
            amount: &BigInt<Self::Api>,
        ) -> ContractCall<Self::Api, <SCResult<()> as elrond_wasm::io::EndpointResult>::DecodeAs>
        {
            let (___api___, ___address___) = self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                &b"add"[..],
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
        A: elrond_wasm::api::VMApi + Clone + 'static,
    {
        api: A,
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::VMApi + Clone + 'static,
    {
        type Api = A;

        fn raw_vm_api(&self) -> Self::Api {
            self.api.clone()
        }
    }

    impl<A> super::module_1::AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::VMApi + Clone + 'static
    {
    }

    impl<A> AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi + Clone + 'static {}

    impl<A> super::module_1::EndpointWrappers for ContractObj<A> where
        A: elrond_wasm::api::VMApi + Clone + 'static
    {
    }

    impl<A> EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi + Clone + 'static {}

    impl<A> elrond_wasm::contract_base::CallableContract<A> for ContractObj<A>
    where
        A: elrond_wasm::api::VMApi + Clone + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }

    pub struct AbiProvider {}

    impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
        type Api = elrond_wasm::api::uncallable::UncallableApi;

        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { build_info : elrond_wasm :: abi :: BuildInfoAbi { contract_crate : elrond_wasm :: abi :: ContractCrateBuildAbi { name : "adder" , version : "0.0.0" , } , framework : elrond_wasm :: abi :: FrameworkBuildAbi :: create () , } , docs : & ["One of the simplest smart contracts possible," , "it holds a single variable in storage, which anyone can increment."] , name : "Adder" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "getSum",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_output::<BigInt<Self::Api>>(&[]);
            contract_abi.add_type_descriptions::<BigInt<Self::Api>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "init",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&BigInt<Self::Api>>("initial_value");
            contract_abi.add_type_descriptions::<&BigInt<Self::Api>>();
            contract_abi.constructor = Some(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &["Add desired amount to the storage variable."],
                name: "add",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&BigInt<Self::Api>>("value");
            contract_abi.add_type_descriptions::<&BigInt<Self::Api>>();
            endpoint_abi.add_output::<SCResult<()>>(&[]);
            contract_abi.add_type_descriptions::<SCResult<()>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi.coalesce(
                <super::module_1::AbiProvider as elrond_wasm::contract_base::ContractAbiProvider>::abi(),
            );
            contract_abi
        }
    }

    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A: elrond_wasm::api::VMApi + Clone + 'static,
    {
        ContractObj { api }
    }

    pub struct Proxy<A>
    where
        A: elrond_wasm::api::VMApi + 'static,
    {
        pub api: A,
        pub address: elrond_wasm::types::ManagedAddress<A>,
    }

    impl<A> elrond_wasm::contract_base::ProxyObjBase for Proxy<A>
    where
        A: elrond_wasm::api::VMApi + 'static,
    {
        type Api = A;

        fn new_proxy_obj(api: A) -> Self {
            let zero_address = ManagedAddress::zero_address(api.clone());
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

    impl<A> super::module_1::ProxyTrait for Proxy<A> where A: elrond_wasm::api::VMApi {}

    impl<A> ProxyTrait for Proxy<A> where A: elrond_wasm::api::VMApi {}

    pub struct CallbackProxyObj<A>
    where
        A: elrond_wasm::api::VMApi + 'static,
    {
        pub api: A,
    }

    impl<A> elrond_wasm::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
    where
        A: elrond_wasm::api::VMApi + 'static,
    {
        type Api = A;

        fn new_cb_proxy_obj(api: A) -> Self {
            CallbackProxyObj { api }
        }
        fn cb_call_api(self) -> Self::Api {
            self.api.clone()
        }
    }

    pub trait CallbackProxy: elrond_wasm::contract_base::CallbackProxyObjBase + Sized {
        fn my_callback(self, caller: &Address) -> elrond_wasm::types::CallbackCall<Self::Api> {
            let mut ___callback_call___ =
                elrond_wasm::types::new_callback_call(self.cb_call_api(), &b"my_callback"[..]);
            ___callback_call___.push_endpoint_arg(caller);
            ___callback_call___
        }
    }
    impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: elrond_wasm::api::VMApi + 'static {}
}

#[test]
fn test_add() {
    use elrond_wasm_debug::TxContext;
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let tx_context = TxContext::dummy();

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
        .contract(ManagedAddress::zero_address(tx_context));
    let _ = own_proxy.get_sum();

    let _ = elrond_wasm_debug::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

fn contract_map() -> elrond_wasm_debug::ContractMap<elrond_wasm_debug::TxContext> {
    let mut contract_map = elrond_wasm_debug::ContractMap::new();
    contract_map.register_contract(
        "file:../output/adder.wasm",
        Box::new(|context| Box::new(sample_adder::contract_obj(context))),
    );
    contract_map
}

#[test]
fn test_mandos() {
    elrond_wasm_debug::mandos_rs(
        "../contracts/examples/adder/mandos/adder.scen.json",
        &contract_map(),
    );
}
