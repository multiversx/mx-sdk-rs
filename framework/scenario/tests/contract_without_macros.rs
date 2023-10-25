// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

#![allow(unused)]

use multiversx_sc::{
    contract_base::ProxyObjBase,
    types::{BigInt, ManagedAddress},
};
use multiversx_sc_scenario::api::{SingleTxApi, StaticApi};

use crate::module_1::VersionModule;

mod module_1 {
    multiversx_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: multiversx_sc::contract_base::ContractBase<CurrentApi> + Sized {
        fn version(&self) -> BigInt<CurrentApi>;

        fn some_async(&self);

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: multiversx_sc::contract_base::ContractBase<CurrentApi> {}

    impl<C> VersionModule for C
    where
        C: AutoImpl,
    {
        fn version(&self) -> BigInt<CurrentApi> {
            BigInt::from(100)
        }

        fn some_async(&self) {
            panic!("wooo")
        }

        fn callback(&self) {}
    }

    impl AutoImpl for multiversx_sc::contract_base::UniversalContractObj<CurrentApi> {}

    pub trait EndpointWrappers: VersionModule + multiversx_sc::contract_base::ContractBase<CurrentApi> {
        #[inline]
        fn call_version(&self) {
            multiversx_sc::io::call_value_init::not_payable::<CurrentApi>();
            let result = self.version();
            multiversx_sc::io::finish_multi::<CurrentApi, _>(&result)
        }

        fn call_some_async(&self) {
            self.some_async();
            multiversx_sc::io::finish_multi::<CurrentApi, _>(&())
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

    impl EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<CurrentApi> {}

    pub struct AbiProvider {}

    impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = multiversx_sc::api::uncallable::UncallableApi;

        fn abi() -> multiversx_sc::abi::ContractAbi {
            multiversx_sc::abi::ContractAbi::default()
        }
    }

    pub trait ProxyTrait<A>: multiversx_sc::contract_base::ProxyObjBase<A> + Sized
    where
        A: multiversx_sc::api::VMApi
    {
        fn version(
            &mut self,
        ) -> multiversx_sc::types::ContractCallNoPayment<A, BigInt<A>> {
            let ___address___ = self.extract_address();
            multiversx_sc::types::ContractCallNoPayment::new(___address___, "version")
        }
    }
}

mod sample_adder {
    multiversx_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait Adder:
        super::module_1::VersionModule + multiversx_sc::contract_base::ContractBase<CurrentApi> + Sized
    {
        fn init(&self, initial_value: &BigInt<CurrentApi>) {
            self.set_sum(initial_value);
        }
        fn add(&self, value: BigInt<CurrentApi>) -> SCResult<()> {
            let mut sum = self.get_sum();
            sum.add_assign(value);
            self.set_sum(&sum);
            Ok(())
        }
        fn get_sum(&self) -> BigInt<CurrentApi>;
        fn set_sum(&self, sum: &BigInt<CurrentApi>);
        fn add_version(&self) -> SCResult<()> {
            self.add(self.version())
        }
        fn callback(&self);
        fn callbacks(&self) -> self::CallbackProxyObj<CurrentApi>;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: multiversx_sc::contract_base::ContractBase<CurrentApi> {}

    // impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        fn get_sum(&self) -> BigInt<CurrentApi> {
            let mut ___key___ = multiversx_sc::storage::StorageKey::<CurrentApi>::new(&b"sum"[..]);
            multiversx_sc::storage_get(multiversx_sc::types::ManagedRef::new(&___key___))
        }
        fn set_sum(&self, sum: &BigInt<CurrentApi>) {
            let mut ___key___ = multiversx_sc::storage::StorageKey::<CurrentApi>::new(&b"sum"[..]);
            multiversx_sc::storage_set(multiversx_sc::types::ManagedRef::new(&___key___), &sum);
        }
        fn callback(&self) {}
        fn callbacks(&self) -> self::CallbackProxyObj<CurrentApi> {
            <self::CallbackProxyObj::<CurrentApi> as multiversx_sc::contract_base::CallbackProxyObjBase>::new_cb_proxy_obj()
        }
    }

    impl AutoImpl for multiversx_sc::contract_base::UniversalContractObj<CurrentApi> {}

    pub trait EndpointWrappers:
        Adder + multiversx_sc::contract_base::ContractBase<CurrentApi> + super::module_1::EndpointWrappers
    {
        #[inline]
        fn call_get_sum(&self) {
            <CurrentApi as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<CurrentApi>();
            let () = multiversx_sc::io::load_endpoint_args::<CurrentApi, ()>(());
            let result = self.get_sum();
            multiversx_sc::io::finish_multi::<CurrentApi, _>(&result);
        }
        #[inline]
        fn call_init(&self) {
            <CurrentApi as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<CurrentApi>();
            let (initial_value, ()) = multiversx_sc::io::load_endpoint_args::<
                CurrentApi,
                (multiversx_sc::types::BigInt<CurrentApi>, ()),
            >(("initial_value", ()));
            self.init(&initial_value);
        }
        #[inline]
        fn call_add(&self) {
            <CurrentApi as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<CurrentApi>();
            let (value, ()) = multiversx_sc::io::load_endpoint_args::<
                CurrentApi,
                (multiversx_sc::types::BigInt<CurrentApi>, ()),
            >(("value", ()));
            let result = self.add(value);
            multiversx_sc::io::finish_multi::<CurrentApi, _>(&result);
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

    impl EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<CurrentApi> {}

    pub trait ProxyTrait<A>:
        multiversx_sc::contract_base::ProxyObjBase<A> + super::module_1::ProxyTrait<A>
    where
        A: multiversx_sc::api::VMApi
    {
        fn get_sum(
            &mut self,
        ) -> multiversx_sc::types::ContractCallNoPayment<A, BigInt<A>> {
            let ___address___ = self.extract_address();
            multiversx_sc::types::ContractCallNoPayment::new(___address___, "get_sum")
        }
        fn add(
            &mut self,
            amount: &BigInt<A>,
        ) -> multiversx_sc::types::ContractCallNoPayment<A, ()> {
            let ___address___ = self.extract_address();
            let mut ___contract_call___ =
                multiversx_sc::types::ContractCallNoPayment::new(___address___, "add");
            multiversx_sc::types::ContractCall::proxy_arg(&mut ___contract_call___, amount);
            ___contract_call___
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub struct ContractObj;

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    impl multiversx_sc::contract_base::ContractBase<CurrentApi> for ContractObj {}

    impl super::module_1::AutoImpl for ContractObj {}

    impl AutoImpl for ContractObj {}

    impl super::module_1::EndpointWrappers for ContractObj {}

    impl EndpointWrappers for ContractObj {}

    impl multiversx_sc::contract_base::CallableContract for ContractObj {
        fn call(&self, fn_name: &str) -> bool {
            EndpointWrappers::call(
                &multiversx_sc::contract_base::UniversalContractObj::<CurrentApi>::new(),
                fn_name,
            )
        }
    }

    pub struct ContractBuilder;

    impl multiversx_sc::contract_base::CallableContractBuilder for ContractBuilder {
        fn new_contract_obj(
            &self,
        ) -> multiversx_sc::types::heap::Box<dyn multiversx_sc::contract_base::CallableContract>
        {
            multiversx_sc::types::heap::Box::new(ContractObj)
        }
    }

    pub struct AbiProvider {}

    impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = multiversx_sc::api::uncallable::UncallableApi;

        fn abi() -> multiversx_sc::abi::ContractAbi {
            multiversx_sc::abi::ContractAbi::default()
        }
    }

    pub fn contract_obj() -> ContractObj {
        ContractObj
    }

    pub struct Proxy<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        pub address:
            multiversx_sc::types::ManagedOption<A, multiversx_sc::types::ManagedAddress<A>>,
    }

    impl<A> multiversx_sc::contract_base::ProxyObjBase<A> for Proxy<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        fn new_proxy_obj() -> Self {
            Proxy {
                address: multiversx_sc::types::ManagedOption::none(),
            }
        }

        fn contract(mut self, address: multiversx_sc::types::ManagedAddress<A>) -> Self {
            self.address = multiversx_sc::types::ManagedOption::some(address);
            self
        }

        fn extract_opt_address(
            &mut self,
        ) -> multiversx_sc::types::ManagedOption<
            A,
            multiversx_sc::types::ManagedAddress<A>,
        > {
            core::mem::replace(
                &mut self.address,
                multiversx_sc::types::ManagedOption::none(),
            )
        }

        fn extract_address(&mut self) -> multiversx_sc::types::ManagedAddress<A> {
            let address = core::mem::replace(
                &mut self.address,
                multiversx_sc::types::ManagedOption::none(),
            );
            address.unwrap_or_sc_panic(multiversx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
        }
    }

    impl<A> super::module_1::ProxyTrait<A> for Proxy<A> where A: multiversx_sc::api::VMApi {}

    impl<A> ProxyTrait<A> for Proxy<A> where A: multiversx_sc::api::VMApi {}

    pub struct CallbackProxyObj<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    impl<A> multiversx_sc::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        type Api = A;

        fn new_cb_proxy_obj() -> Self {
            CallbackProxyObj {
                _phantom: core::marker::PhantomData,
            }
        }
    }

    pub trait CallbackProxy: multiversx_sc::contract_base::CallbackProxyObjBase + Sized {
        fn my_callback(self, caller: &Address) -> multiversx_sc::types::CallbackClosure<CurrentApi> {
            let mut ___callback_call___ =
                multiversx_sc::types::new_callback_call::<CurrentApi>("my_callback");
            ___callback_call___.push_endpoint_arg(caller);
            ___callback_call___
        }
    }
    impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: multiversx_sc::api::VMApi + 'static {}
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn contract_without_macros_basic() {
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let adder = sample_adder::contract_obj();

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
        sample_adder::Proxy::<StaticApi>::new_proxy_obj().contract(ManagedAddress::zero());
    let _ = own_proxy.get_sum();

    let _ = multiversx_sc_meta::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

fn world() -> multiversx_sc_scenario::ScenarioWorld {
    let mut blockchain = multiversx_sc_scenario::ScenarioWorld::new();
    blockchain.register_contract(
        "file:../../contracts/examples/adder/output/adder.wasm",
        sample_adder::ContractBuilder,
    );
    blockchain
}

#[test]
fn contract_without_macros_scenario() {
    world().run("../../contracts/examples/adder/scenarios/adder.scen.json");
}
