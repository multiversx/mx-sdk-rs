// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

use mx_sc::{
    contract_base::ProxyObjBase,
    types::{BigInt, ManagedAddress},
};

use crate::module_1::VersionModule;

mod module_1 {
    mx_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: mx_sc::contract_base::ContractBase + Sized {
        fn version(&self) -> BigInt<Self::Api>;

        fn some_async(&self);

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: mx_sc::contract_base::ContractBase {}

    impl<C> VersionModule for C
    where
        C: AutoImpl,
    {
        fn version(&self) -> BigInt<Self::Api> {
            BigInt::from(100)
        }

        fn some_async(&self) {
            panic!("wooo")
        }

        fn callback(&self) {}
    }

    impl<A> AutoImpl for mx_sc::contract_base::UniversalContractObj<A> where A: mx_sc::api::VMApi {}

    pub trait EndpointWrappers: VersionModule + mx_sc::contract_base::ContractBase {
        #[inline]
        fn call_version(&self) {
            mx_sc::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
            let result = self.version();
            mx_sc::io::finish_multi::<Self::Api, _>(&result)
        }

        fn call_some_async(&self) {
            self.some_async();
            mx_sc::io::finish_multi::<Self::Api, _>(&())
        }

        fn call(&self, fn_name: &str) -> bool {
            if match fn_name {
                "callBack" => {
                    self.callback();
                    return true;
                },
                "version" => {
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

    impl<A> EndpointWrappers for mx_sc::contract_base::UniversalContractObj<A> where A: mx_sc::api::VMApi
    {}

    pub struct AbiProvider {}

    impl mx_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = mx_sc::api::uncallable::UncallableApi;

        fn abi() -> mx_sc::abi::ContractAbi {
            mx_sc::abi::ContractAbi::default()
        }
    }

    pub trait ProxyTrait: mx_sc::contract_base::ProxyObjBase + Sized {
        fn version(&mut self) -> mx_sc::types::ContractCallNoPayment<Self::Api, BigInt<Self::Api>> {
            let ___address___ = self.extract_address();
            mx_sc::types::ContractCallNoPayment::new(___address___, "version")
        }
    }
}

mod sample_adder {
    mx_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait Adder:
        super::module_1::VersionModule + mx_sc::contract_base::ContractBase + Sized
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
    pub trait AutoImpl: mx_sc::contract_base::ContractBase {}

    // impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        fn get_sum(&self) -> BigInt<Self::Api> {
            let mut ___key___ = mx_sc::storage::StorageKey::<Self::Api>::new(&b"sum"[..]);
            mx_sc::storage_get(mx_sc::types::ManagedRef::new(&___key___))
        }
        fn set_sum(&self, sum: &BigInt<Self::Api>) {
            let mut ___key___ = mx_sc::storage::StorageKey::<Self::Api>::new(&b"sum"[..]);
            mx_sc::storage_set(mx_sc::types::ManagedRef::new(&___key___), &sum);
        }
        fn callback(&self) {}
        fn callbacks(&self) -> self::CallbackProxyObj<Self::Api> {
            <self::CallbackProxyObj::<Self::Api> as mx_sc::contract_base::CallbackProxyObjBase>::new_cb_proxy_obj()
        }
    }

    impl<A> AutoImpl for mx_sc::contract_base::UniversalContractObj<A> where A: mx_sc::api::VMApi {}

    pub trait EndpointWrappers:
        Adder + mx_sc::contract_base::ContractBase + super::module_1::EndpointWrappers
    {
        #[inline]
        fn call_get_sum(&self) {
            <Self::Api as mx_sc::api::VMApi>::init_static();
            mx_sc::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
            let () = mx_sc::io::load_endpoint_args::<Self::Api, ()>(());
            let result = self.get_sum();
            mx_sc::io::finish_multi::<Self::Api, _>(&result);
        }
        #[inline]
        fn call_init(&self) {
            <Self::Api as mx_sc::api::VMApi>::init_static();
            mx_sc::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
            let (initial_value, ()) = mx_sc::io::load_endpoint_args::<
                Self::Api,
                (mx_sc::types::BigInt<Self::Api>, ()),
            >(("initial_value", ()));
            self.init(&initial_value);
        }
        #[inline]
        fn call_add(&self) {
            <Self::Api as mx_sc::api::VMApi>::init_static();
            mx_sc::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
            let (value, ()) = mx_sc::io::load_endpoint_args::<
                Self::Api,
                (mx_sc::types::BigInt<Self::Api>, ()),
            >(("value", ()));
            let result = self.add(value);
            mx_sc::io::finish_multi::<Self::Api, _>(&result);
        }

        fn call(&self, fn_name: &str) -> bool {
            if match fn_name {
                "callBack" => {
                    Adder::callback(self);
                    return true;
                },
                "getSum" => {
                    self.call_get_sum();
                    true
                },
                "init" => {
                    self.call_init();
                    true
                },
                "add" => {
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

    impl<A> EndpointWrappers for mx_sc::contract_base::UniversalContractObj<A> where A: mx_sc::api::VMApi
    {}

    pub trait ProxyTrait: mx_sc::contract_base::ProxyObjBase + super::module_1::ProxyTrait {
        fn get_sum(&mut self) -> mx_sc::types::ContractCallNoPayment<Self::Api, BigInt<Self::Api>> {
            let ___address___ = self.extract_address();
            mx_sc::types::ContractCallNoPayment::new(___address___, "get_sum")
        }
        fn add(
            &mut self,
            amount: &BigInt<Self::Api>,
        ) -> mx_sc::types::ContractCallNoPayment<Self::Api, ()> {
            let ___address___ = self.extract_address();
            let mut ___contract_call___ =
                mx_sc::types::ContractCallNoPayment::new(___address___, "add");
            mx_sc::types::ContractCall::proxy_arg(&mut ___contract_call___, amount);
            ___contract_call___
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub struct ContractObj<A>
    where
        A: mx_sc::api::VMApi,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    impl<A> mx_sc::contract_base::ContractBase for ContractObj<A>
    where
        A: mx_sc::api::VMApi,
    {
        type Api = A;
    }

    impl<A> super::module_1::AutoImpl for ContractObj<A> where A: mx_sc::api::VMApi {}

    impl<A> AutoImpl for ContractObj<A> where A: mx_sc::api::VMApi {}

    impl<A> super::module_1::EndpointWrappers for ContractObj<A> where A: mx_sc::api::VMApi {}

    impl<A> EndpointWrappers for ContractObj<A> where A: mx_sc::api::VMApi {}

    impl<A> mx_sc::contract_base::CallableContract for ContractObj<A>
    where
        A: mx_sc::api::VMApi,
    {
        fn call(&self, fn_name: &str) -> bool {
            EndpointWrappers::call(
                &mx_sc::contract_base::UniversalContractObj::<A>::new(),
                fn_name,
            )
        }
    }

    pub struct ContractBuilder;

    impl mx_sc::contract_base::CallableContractBuilder for ContractBuilder {
        fn new_contract_obj<A: mx_sc::api::VMApi>(
            &self,
        ) -> mx_sc::types::heap::Box<dyn mx_sc::contract_base::CallableContract> {
            mx_sc::types::heap::Box::new(ContractObj::<A> {
                _phantom: core::marker::PhantomData,
            })
        }
    }

    pub struct AbiProvider {}

    impl mx_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = mx_sc::api::uncallable::UncallableApi;

        fn abi() -> mx_sc::abi::ContractAbi {
            mx_sc::abi::ContractAbi::default()
        }
    }

    pub fn contract_obj<A>() -> ContractObj<A>
    where
        A: mx_sc::api::VMApi,
    {
        ContractObj {
            _phantom: core::marker::PhantomData,
        }
    }

    pub struct Proxy<A>
    where
        A: mx_sc::api::VMApi + 'static,
    {
        pub address: mx_sc::types::ManagedOption<A, mx_sc::types::ManagedAddress<A>>,
    }

    impl<A> mx_sc::contract_base::ProxyObjBase for Proxy<A>
    where
        A: mx_sc::api::VMApi + 'static,
    {
        type Api = A;

        fn new_proxy_obj() -> Self {
            Proxy {
                address: mx_sc::types::ManagedOption::none(),
            }
        }

        fn contract(mut self, address: mx_sc::types::ManagedAddress<Self::Api>) -> Self {
            self.address = mx_sc::types::ManagedOption::some(address);
            self
        }

        fn extract_opt_address(
            &mut self,
        ) -> mx_sc::types::ManagedOption<Self::Api, mx_sc::types::ManagedAddress<Self::Api>>
        {
            core::mem::replace(&mut self.address, mx_sc::types::ManagedOption::none())
        }

        fn extract_address(&mut self) -> mx_sc::types::ManagedAddress<Self::Api> {
            let address =
                core::mem::replace(&mut self.address, mx_sc::types::ManagedOption::none());
            address.unwrap_or_sc_panic(mx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
        }
    }

    impl<A> super::module_1::ProxyTrait for Proxy<A> where A: mx_sc::api::VMApi {}

    impl<A> ProxyTrait for Proxy<A> where A: mx_sc::api::VMApi {}

    pub struct CallbackProxyObj<A>
    where
        A: mx_sc::api::VMApi + 'static,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    impl<A> mx_sc::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
    where
        A: mx_sc::api::VMApi + 'static,
    {
        type Api = A;

        fn new_cb_proxy_obj() -> Self {
            CallbackProxyObj {
                _phantom: core::marker::PhantomData,
            }
        }
    }

    pub trait CallbackProxy: mx_sc::contract_base::CallbackProxyObjBase + Sized {
        fn my_callback(self, caller: &Address) -> mx_sc::types::CallbackClosure<Self::Api> {
            let mut ___callback_call___ =
                mx_sc::types::new_callback_call::<Self::Api>("my_callback");
            ___callback_call___.push_endpoint_arg(caller);
            ___callback_call___
        }
    }
    impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: mx_sc::api::VMApi + 'static {}
}

#[test]
fn test_add() {
    use mx_sc_debug::DebugApi;
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let _ = DebugApi::dummy();

    let adder = sample_adder::contract_obj::<DebugApi>();

    adder.init(&BigInt::from(5));
    assert_eq!(BigInt::from(5), adder.get_sum());

    let _ = adder.add(BigInt::from(7));
    assert_eq!(BigInt::from(12), adder.get_sum());

    let _ = adder.add(BigInt::from(-1));
    assert_eq!(BigInt::from(11), adder.get_sum());

    assert_eq!(BigInt::from(100), adder.version());

    let _ = adder.add_version();
    assert_eq!(BigInt::from(111), adder.get_sum());

    assert!(!adder.call("invalid_endpoint"));

    assert!(adder.call("version"));

    let mut own_proxy =
        sample_adder::Proxy::<DebugApi>::new_proxy_obj().contract(ManagedAddress::zero());
    let _ = own_proxy.get_sum();

    let _ = mx_sc_meta::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

fn world() -> mx_sc_debug::BlockchainMock {
    let mut blockchain = mx_sc_debug::BlockchainMock::new();
    blockchain.register_contract(
        "file:../../contracts/examples/adder/output/adder.wasm",
        sample_adder::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_mandos() {
    mx_sc_debug::mandos_rs(
        "../../contracts/examples/adder/mandos/adder.scen.json",
        world(),
    );
}
