// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

use elrond_wasm::types::{Address, BigInt};

use crate::module_1::VersionModule;

mod module_1 {
    elrond_wasm::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: elrond_wasm::api::ContractBase + Sized {
        fn version(&self) -> BigInt<Self::TypeManager>;

        fn some_async(&self) -> AsyncCall<Self::SendApi>;

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}

    impl<C> VersionModule for C
    where
        C: AutoImpl,
    {
        fn version(&self) -> BigInt<Self::TypeManager> {
            BigInt::from_i64(100, self.type_manager())
        }

        fn some_async(&self) -> AsyncCall<Self::SendApi> {
            panic!("wooo")
        }

        fn callback(&self) {}
    }

    pub trait EndpointWrappers: VersionModule + elrond_wasm::api::ContractPrivateApi {
        #[inline]
        fn call_version(&self) {
            self.call_value().check_not_payable();
            let result = self.version();
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api())
        }

        fn call_some_async(&self) {
            let result = self.some_async();
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api())
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

    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type TypeManager = elrond_wasm::api::uncallable::UncallableApi;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;

        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [ "One of the simplest smart contracts possible," , "it holds a single variable in storage, which anyone can increment." ] , name : "Adder" , constructor : None , endpoints : Vec :: new ( ) , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new ( ) , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "version",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_output::<BigInt<Self::TypeManager>>(&[]);
            contract_abi.add_type_descriptions::<BigInt<Self::TypeManager>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }

    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
        fn version(
            self,
        ) -> ContractCall<
            Self::SendApi,
            <BigInt<Self::TypeManager> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"version"[..]),
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
        super::module_1::VersionModule + elrond_wasm::api::ContractBase + Sized
    {
        fn init(&self, initial_value: &BigInt<Self::TypeManager>) {
            self.set_sum(initial_value);
        }
        fn add(&self, value: BigInt<Self::TypeManager>) -> SCResult<()> {
            let mut sum = self.get_sum();
            sum.add_assign(value);
            self.set_sum(&sum);
            Ok(())
        }
        fn get_sum(&self) -> BigInt<Self::TypeManager>;
        fn set_sum(&self, sum: &BigInt<Self::TypeManager>);
        fn add_version(&self) -> SCResult<()> {
            self.add(self.version())
        }
        fn callback(&self);
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi>;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}

    // impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        fn get_sum(&self) -> BigInt<Self::TypeManager> {
            let key: &'static [u8] = b"sum";
            elrond_wasm::storage_get(self.get_storage_raw(), &key[..])
        }
        fn set_sum(&self, sum: &BigInt<Self::TypeManager>) {
            let key: &'static [u8] = b"sum";
            elrond_wasm::storage_set(self.get_storage_raw(), &key[..], &sum);
        }
        fn callback(&self) {}
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi> {
            <self::CallbackProxyObj::<Self::SendApi> as elrond_wasm::api::CallbackProxyObjApi>::new_cb_proxy_obj(self.send())
        }
    }

    pub trait EndpointWrappers:
        Adder + elrond_wasm::api::ContractPrivateApi + super::module_1::EndpointWrappers
    {
        #[inline]
        fn call_get_sum(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
            let result = self.get_sum();
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_init(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let initial_value =
                elrond_wasm::load_single_arg::<Self::ArgumentApi, BigInt<Self::TypeManager>>(
                    self.argument_api(),
                    0i32,
                    ArgId::from(&b"initial_value"[..]),
                );
            self.init(&initial_value);
        }
        #[inline]
        fn call_add(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let value = elrond_wasm::load_single_arg::<Self::ArgumentApi, BigInt<Self::TypeManager>>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"value"[..]),
            );
            let result = self.add(value);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
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

    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + super::module_1::ProxyTrait {
        fn get_sum(
            self,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <BigInt<Self::TypeManager> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"get_sum"[..]),
            );
            ___contract_call___
        }
        fn add(
            self,
            amount: &BigInt<Self::TypeManager>,
        ) -> ContractCall<Self::SendApi, <SCResult<()> as elrond_wasm::io::EndpointResult>::DecodeAs>
        {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"add"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + elrond_wasm::api::ManagedTypeApi
            + Clone
            + 'static,
    {
        type TypeManager = A::TypeManager;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;

        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn type_manager(&self) -> Self::TypeManager {
            self.api.type_manager()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }

    impl<A> super::module_1::AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + elrond_wasm::api::ManagedTypeApi
            + Clone
            + 'static
    {
    }

    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + elrond_wasm::api::ManagedTypeApi
            + Clone
            + 'static
    {
    }

    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + elrond_wasm::api::ManagedTypeApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type CallbackClosureArgumentApi = A;
        type FinishApi = A;

        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }

        #[inline]
        fn callback_closure_arg_api(&self) -> Self::CallbackClosureArgumentApi {
            self.api.clone()
        }

        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }

    impl<A> super::module_1::EndpointWrappers for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + elrond_wasm::api::ManagedTypeApi
            + Clone
            + 'static
    {
    }

    impl<A> EndpointWrappers for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + elrond_wasm::api::ManagedTypeApi
            + Clone
            + 'static
    {
    }

    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + elrond_wasm::api::ManagedTypeApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }

    pub struct AbiProvider {}

    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type TypeManager = elrond_wasm::api::uncallable::UncallableApi;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;

        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [ "One of the simplest smart contracts possible," , "it holds a single variable in storage, which anyone can increment." ] , name : "Adder" , constructor : None , endpoints : Vec :: new ( ) , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new ( ) , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "getSum",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_output::<BigInt<Self::TypeManager>>(&[]);
            contract_abi.add_type_descriptions::<BigInt<Self::TypeManager>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "init",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&BigInt<Self::TypeManager>>("initial_value");
            contract_abi.add_type_descriptions::<&BigInt<Self::TypeManager>>();
            contract_abi.constructor = Some(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &["Add desired amount to the storage variable."],
                name: "add",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&BigInt<Self::TypeManager>>("value");
            contract_abi.add_type_descriptions::<&BigInt<Self::TypeManager>>();
            endpoint_abi.add_output::<SCResult<()>>(&[]);
            contract_abi.add_type_descriptions::<SCResult<()>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi.coalesce(
                <super::module_1::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi(),
            );
            contract_abi
        }
    }

    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }

    pub struct Proxy<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        pub api: SA,
        pub address: Address,
        pub payment_token: elrond_wasm::types::TokenIdentifier,
        pub payment_amount: elrond_wasm::types::BigUint<SA::ProxyTypeManager>,
        pub payment_nonce: u64,
    }

    impl<SA> elrond_wasm::api::ProxyObjApi for Proxy<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        type TypeManager = SA::ProxyTypeManager;
        type Storage = SA::ProxyStorage;
        type SendApi = SA;

        fn new_proxy_obj(api: SA) -> Self {
            let zero = elrond_wasm::types::BigUint::zero(api.type_manager());
            Proxy {
                api,
                address: Address::zero(),
                payment_token: elrond_wasm::types::TokenIdentifier::egld(),
                payment_amount: zero,
                payment_nonce: 0,
            }
        }

        fn contract(mut self, address: Address) -> Self {
            self.address = address;
            self
        }

        fn with_token_transfer(
            mut self,
            token: TokenIdentifier,
            payment: BigUint<Self::TypeManager>,
        ) -> Self {
            self.payment_token = token;
            self.payment_amount = payment;
            self
        }

        #[inline]
        fn with_nft_nonce(mut self, nonce: u64) -> Self {
            self.payment_nonce = nonce;
            self
        }

        #[inline]
        fn into_fields(
            self,
        ) -> (
            Self::SendApi,
            Address,
            TokenIdentifier,
            BigUint<Self::TypeManager>,
            u64,
        ) {
            (
                self.api,
                self.address,
                self.payment_token,
                self.payment_amount,
                self.payment_nonce,
            )
        }
    }

    impl<SA> super::module_1::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}

    impl<SA> ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}

    pub fn new_proxy_obj<SA>(api: SA, address: Address) -> impl ProxyTrait
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        Proxy::new_proxy_obj(api).contract(address)
    }

    pub struct CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        pub api: SA,
    }
    impl<SA> elrond_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        type TypeManager = SA::ProxyTypeManager;
        type Storage = SA::ProxyStorage;
        type SendApi = SA;
        type ErrorApi = SA;
        fn new_cb_proxy_obj(api: SA) -> Self {
            CallbackProxyObj { api }
        }
        fn into_api(self) -> Self::ErrorApi {
            self.api
        }
    }

    pub trait CallbackProxy: elrond_wasm::api::CallbackProxyObjApi + Sized {
        fn my_callback(self, caller: &Address) -> elrond_wasm::types::CallbackCall {
            let ___api___ = self.into_api();
            let mut ___closure_arg_buffer___ = elrond_wasm::types::ArgBuffer::new();
            elrond_wasm::io::serialize_contract_call_arg(
                caller,
                &mut ___closure_arg_buffer___,
                ___api___.clone(),
            );
            elrond_wasm::types::CallbackCall::from_arg_buffer(
                &b"my_callback"[..],
                &___closure_arg_buffer___,
            )
        }
    }
    impl<SA> self::CallbackProxy for CallbackProxyObj<SA> where SA: elrond_wasm::api::SendApi + 'static {}
}

#[test]
fn test_add() {
    use elrond_wasm::api::ContractBase;
    use elrond_wasm_debug::TxContext;
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let tx_context = TxContext::dummy();

    let adder = sample_adder::contract_obj(tx_context.clone());

    adder.init(&BigInt::from_i64(5, adder.type_manager()));
    assert_eq!(BigInt::from_i64(5, adder.type_manager()), adder.get_sum());

    let _ = adder.add(BigInt::from_i64(7, adder.type_manager()));
    assert_eq!(BigInt::from_i64(12, adder.type_manager()), adder.get_sum());

    let _ = adder.add(BigInt::from_i64(-1, adder.type_manager()));
    assert_eq!(BigInt::from_i64(11, adder.type_manager()), adder.get_sum());

    assert_eq!(BigInt::from_i64(100, adder.type_manager()), adder.version());

    let _ = adder.add_version();
    assert_eq!(BigInt::from_i64(111, adder.type_manager()), adder.get_sum());

    assert!(!adder.call(b"invalid_endpoint"));

    assert!(adder.call(b"version"));

    let own_proxy = sample_adder::new_proxy_obj(adder.send(), Address::zero());
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
