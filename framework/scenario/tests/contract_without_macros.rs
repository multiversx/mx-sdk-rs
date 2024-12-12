// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

#![allow(unused)]

use multiversx_sc::{
    contract_base::{CallableContractBuilder, ProxyObjNew},
    types::{BigInt, ManagedAddress},
};
use multiversx_sc_scenario::api::{SingleTxApi, StaticApi};

use crate::module_1::VersionModule;

mod module_1 {
    multiversx_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: multiversx_sc::contract_base::ContractBase + Sized {
        fn version(&self) -> BigInt<Self::Api>;

        fn some_async(&self);

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: multiversx_sc::contract_base::ContractBase {}

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

    impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A> where
        A: multiversx_sc::api::VMApi
    {
    }

    pub trait EndpointWrappers: VersionModule + multiversx_sc::contract_base::ContractBase {
        #[inline]
        fn call_version(&mut self) {
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let result = self.version();
            multiversx_sc::io::finish_multi::<Self::Api, _>(&result)
        }

        fn call_some_async(&mut self) {
            self.some_async();
            multiversx_sc::io::finish_multi::<Self::Api, _>(&())
        }

        fn call(&mut self, fn_name: &str) -> bool {
            match fn_name {
                "callBack" => {
                    self.callback();
                    return true;
                },
                "version" => {
                    self.call_version();
                    true
                },
                _other => false,
            }
        }
        fn callback_selector(
            &mut self,
            ___cb_closure___: &multiversx_sc::types::CallbackClosureForDeser<Self::Api>,
        ) -> multiversx_sc::types::CallbackSelectorResult {
            multiversx_sc::types::CallbackSelectorResult::NotProcessed
        }
    }

    impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A> where
        A: multiversx_sc::api::VMApi
    {
    }

    pub struct AbiProvider {}

    impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = multiversx_sc::api::uncallable::UncallableApi;

        fn abi() -> multiversx_sc::abi::ContractAbi {
            multiversx_sc::abi::ContractAbi::default()
        }
    }

    pub trait ProxyTrait: multiversx_sc::contract_base::ProxyObjBase + Sized {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn version(
            &mut self,
        ) -> multiversx_sc::types::Tx<
            multiversx_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            multiversx_sc::types::FunctionCall<Self::Api>,
            multiversx_sc::types::OriginalResultMarker<BigInt<Self::Api>>,
        > {
            multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("version")
        }
    }
}

mod sampler_adder_proxy {
    #![allow(dead_code)]
    #![allow(clippy::all)]
    use multiversx_sc::proxy_imports::*;
    pub struct AdderProxy;
    impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for AdderProxy
    where
        Env: TxEnv,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        type TxProxyMethods = AdderProxyMethods<Env, From, To, Gas>;
        fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
            AdderProxyMethods { wrapped_tx: tx }
        }
    }
    pub struct AdderProxyMethods<Env, From, To, Gas>
    where
        Env: TxEnv,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
    }
    #[rustfmt::skip]
    impl<Env, From, Gas> AdderProxyMethods<Env, From, (), Gas>
    where
        Env: TxEnv,
        Env::Api: VMApi,
        From: TxFrom<Env>,
        Gas: TxGas<Env>,
    {
        pub fn init<Arg0: ProxyArg<BigUint<Env::Api>>>(
            self,
            initial_value: Arg0,
        ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_deploy()
                .argument(&initial_value)
                .original_result()
        }
    }
    #[rustfmt::skip]
    impl<Env, From, To, Gas> AdderProxyMethods<Env, From, To, Gas>
    where
        Env: TxEnv,
        Env::Api: VMApi,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        pub fn upgrade<Arg0: ProxyArg<BigUint<Env::Api>>>(
            self,
            initial_value: Arg0,
        ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_upgrade()
                .argument(&initial_value)
                .original_result()
        }
    }
    #[rustfmt::skip]
    impl<Env, From, To, Gas> AdderProxyMethods<Env, From, To, Gas>
    where
        Env: TxEnv,
        Env::Api: VMApi,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        pub fn sum(
            self,
        ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
            self.wrapped_tx.payment(NotPayable).raw_call("getSum").original_result()
        }
        /// Add desired amount to the storage variable.
        pub fn add<Arg0: ProxyArg<BigUint<Env::Api>>>(
            self,
            value: Arg0,
        ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_call("add")
                .argument(&value)
                .original_result()
        }
    }
}

mod sample_adder {
    multiversx_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait Adder:
        super::module_1::VersionModule + multiversx_sc::contract_base::ContractBase + Sized
    {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn init(&self, initial_value: multiversx_sc::types::BigUint<Self::Api>) {
            self.sum().set(initial_value);
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn upgrade(&self, initial_value: multiversx_sc::types::BigUint<Self::Api>) {
            self.init(initial_value);
        }
        /// Add desired amount to the storage variable.
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn add(&self, value: multiversx_sc::types::BigUint<Self::Api>) {
            self.sum().update(|sum| *sum += value);
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn sum(&self) -> SingleValueMapper<Self::Api, multiversx_sc::types::BigUint<Self::Api>>;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: multiversx_sc::contract_base::ContractBase {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn sum(&self) -> SingleValueMapper<Self::Api, multiversx_sc::types::BigUint<Self::Api>> {
            let mut ___key___ = multiversx_sc::storage::StorageKey::<Self::Api>::new(&b"sum"[..]);
            <SingleValueMapper<
            Self::Api,
            multiversx_sc::types::BigUint<Self::Api>,
        > as multiversx_sc::storage::mappers::StorageMapper<Self::Api>>::new(___key___)
        }
    }

    impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A> where
        A: multiversx_sc::api::VMApi
    {
    }

    pub trait EndpointWrappers:
        Adder + multiversx_sc::contract_base::ContractBase + super::module_1::EndpointWrappers
    {
        #[inline]
        fn call_sum(&mut self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
            let result = self.sum();
            multiversx_sc::io::finish_multi::<Self::Api, _>(&result);
        }
        #[inline]
        fn call_init(&mut self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let (initial_value, ()) = multiversx_sc::io::load_endpoint_args::<
                Self::Api,
                (multiversx_sc::types::BigUint<Self::Api>, ()),
            >(("initial_value", ()));
            self.init(initial_value);
        }
        #[inline]
        fn call_upgrade(&mut self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let (initial_value, ()) = multiversx_sc::io::load_endpoint_args::<
                Self::Api,
                (multiversx_sc::types::BigUint<Self::Api>, ()),
            >(("initial_value", ()));
            self.upgrade(initial_value);
        }
        #[inline]
        fn call_add(&mut self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let (value, ()) = multiversx_sc::io::load_endpoint_args::<
                Self::Api,
                (multiversx_sc::types::BigUint<Self::Api>, ()),
            >(("value", ()));
            self.add(value);
        }
        fn call(&mut self, fn_name: &str) -> bool {
            match fn_name {
                "callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                },
                "init"
                    if <Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() =>
                {
                    multiversx_sc::external_view_contract::external_view_contract_constructor::<
                        Self::Api,
                    >();
                    return true;
                },
                "getSum" => {
                    self.call_sum();
                    true
                },
                "init"
                    if !<Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() =>
                {
                    self.call_init();
                    true
                },
                "upgrade" => {
                    self.call_upgrade();
                    true
                },
                "add" => {
                    self.call_add();
                    true
                },
                other => {
                    if super::module_1::EndpointWrappers::call(self, fn_name) {
                        return true;
                    }
                    false
                },
            }
        }
        fn callback_selector(
            &mut self,
            ___cb_closure___: &multiversx_sc::types::CallbackClosureForDeser<Self::Api>,
        ) -> multiversx_sc::types::CallbackSelectorResult {
            let ___cb_closure_matcher___ = ___cb_closure___.matcher::<32usize>();
            if ___cb_closure_matcher___.matches_empty() {
                return multiversx_sc::types::CallbackSelectorResult::Processed;
            }
            if super::module_1::EndpointWrappers::callback_selector(self, ___cb_closure___)
                .is_processed()
            {
                return multiversx_sc::types::CallbackSelectorResult::Processed;
            }
            multiversx_sc::types::CallbackSelectorResult::NotProcessed
        }
        fn callback(&mut self) {
            if let Some(___cb_closure___) =
                multiversx_sc::types::CallbackClosureForDeser::storage_load_and_clear::<Self::Api>()
            {
                if !self::EndpointWrappers::callback_selector(self, &___cb_closure___)
                    .is_processed()
                {
                    multiversx_sc::api::ErrorApiImpl::signal_error(
                        &<Self::Api as multiversx_sc::api::ErrorApi>::error_api_impl(),
                        err_msg::CALLBACK_BAD_FUNC.as_bytes(),
                    );
                }
            }
        }
    }

    impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A> where
        A: multiversx_sc::api::VMApi
    {
    }

    pub struct AbiProvider {}
    impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = multiversx_sc::api::uncallable::UncallableApi;
        fn abi() -> multiversx_sc::abi::ContractAbi {
            let mut contract_abi = multiversx_sc::abi::ContractAbi::new(
                multiversx_sc::abi::BuildInfoAbi {
                    contract_crate: multiversx_sc::abi::ContractCrateBuildAbi {
                        name: "adder",
                        version: "0.0.0",
                        git_version: "",
                    },
                    framework: multiversx_sc::abi::FrameworkBuildAbi::create(),
                },
                &[
                    "One of the simplest smart contracts possible,",
                    "it holds a single variable in storage, which anyone can increment.",
                ],
                "Adder",
                false,
            );
            let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
                "getSum",
                "sum",
                multiversx_sc::abi::EndpointMutabilityAbi::Readonly,
                multiversx_sc::abi::EndpointTypeAbi::Endpoint,
            );
            endpoint_abi
            .add_output::<
                SingleValueMapper<Self::Api, multiversx_sc::types::BigUint<Self::Api>>,
            >(&[]);
            contract_abi
            .add_type_descriptions::<
                SingleValueMapper<Self::Api, multiversx_sc::types::BigUint<Self::Api>>,
            >();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
                "init",
                "init",
                multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                multiversx_sc::abi::EndpointTypeAbi::Init,
            );
            endpoint_abi.add_input::<multiversx_sc::types::BigUint<Self::Api>>("initial_value");
            contract_abi.add_type_descriptions::<multiversx_sc::types::BigUint<Self::Api>>();
            contract_abi.constructors.push(endpoint_abi);
            let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
                "upgrade",
                "upgrade",
                multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                multiversx_sc::abi::EndpointTypeAbi::Upgrade,
            );
            endpoint_abi.add_input::<multiversx_sc::types::BigUint<Self::Api>>("initial_value");
            contract_abi.add_type_descriptions::<multiversx_sc::types::BigUint<Self::Api>>();
            contract_abi.upgrade_constructors.push(endpoint_abi);
            let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
                "add",
                "add",
                multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                multiversx_sc::abi::EndpointTypeAbi::Endpoint,
            )
            .with_docs("Add desired amount to the storage variable.");
            endpoint_abi.add_input::<multiversx_sc::types::BigUint<Self::Api>>("value");
            contract_abi.add_type_descriptions::<multiversx_sc::types::BigUint<Self::Api>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }

    #[allow(non_snake_case)]
    pub mod __wasm__endpoints__ {
        use super::EndpointWrappers;
        pub fn sum<A>()
        where
            A: multiversx_sc::api::VMApi,
        {
            super::EndpointWrappers::call_sum(
                &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
        pub fn init<A>()
        where
            A: multiversx_sc::api::VMApi,
        {
            super::EndpointWrappers::call_init(
                &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
        pub fn upgrade<A>()
        where
            A: multiversx_sc::api::VMApi,
        {
            super::EndpointWrappers::call_upgrade(
                &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
        pub fn add<A>()
        where
            A: multiversx_sc::api::VMApi,
        {
            super::EndpointWrappers::call_add(
                &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
        pub fn callBack<A>()
        where
            A: multiversx_sc::api::VMApi,
        {
            super::EndpointWrappers::callback(
                &mut multiversx_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
    }
    pub trait ProxyTrait:
        multiversx_sc::contract_base::ProxyObjBase + super::module_1::ProxyTrait
    {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn sum(
            &mut self,
        ) -> multiversx_sc::types::Tx<
            multiversx_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            multiversx_sc::types::FunctionCall<Self::Api>,
            multiversx_sc::types::OriginalResultMarker<
                SingleValueMapper<Self::Api, multiversx_sc::types::BigUint<Self::Api>>,
            >,
        > {
            multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("getSum")
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn init<Arg0: multiversx_sc::types::ProxyArg<multiversx_sc::types::BigUint<Self::Api>>>(
            &mut self,
            initial_value: Arg0,
        ) -> multiversx_sc::types::Tx<
            multiversx_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            multiversx_sc::types::DeployCall<multiversx_sc::types::TxScEnv<Self::Api>, ()>,
            multiversx_sc::types::OriginalResultMarker<()>,
        > {
            multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .raw_deploy()
                .argument(&initial_value)
                .original_result()
                .to(self.extract_proxy_to())
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn upgrade<
            Arg0: multiversx_sc::types::ProxyArg<multiversx_sc::types::BigUint<Self::Api>>,
        >(
            &mut self,
            initial_value: Arg0,
        ) -> multiversx_sc::types::Tx<
            multiversx_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            multiversx_sc::types::FunctionCall<Self::Api>,
            multiversx_sc::types::OriginalResultMarker<()>,
        > {
            multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("upgrade")
                .argument(&initial_value)
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn add<Arg0: multiversx_sc::types::ProxyArg<multiversx_sc::types::BigUint<Self::Api>>>(
            &mut self,
            value: Arg0,
        ) -> multiversx_sc::types::Tx<
            multiversx_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            multiversx_sc::types::FunctionCall<Self::Api>,
            multiversx_sc::types::OriginalResultMarker<()>,
        > {
            multiversx_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("add")
                .argument(&value)
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub struct ContractObj<A>
    where
        A: multiversx_sc::api::VMApi,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    impl<A> multiversx_sc::contract_base::ContractBase for ContractObj<A>
    where
        A: multiversx_sc::api::VMApi,
    {
        type Api = A;
    }

    impl<A> super::module_1::AutoImpl for ContractObj<A> where A: multiversx_sc::api::VMApi {}

    impl<A> AutoImpl for ContractObj<A> where A: multiversx_sc::api::VMApi {}

    impl<A> super::module_1::EndpointWrappers for ContractObj<A> where A: multiversx_sc::api::VMApi {}

    impl<A> EndpointWrappers for ContractObj<A> where A: multiversx_sc::api::VMApi {}

    impl<A> multiversx_sc::contract_base::CallableContract for ContractObj<A>
    where
        A: multiversx_sc::api::VMApi,
    {
        fn call(&self, fn_name: &str) -> bool {
            // creating a new object, which we can mutate
            // because of dynamic traits, we cannot move `self`
            let mut obj = multiversx_sc::contract_base::UniversalContractObj::<A>::new();
            EndpointWrappers::call(&mut obj, fn_name)
        }
    }

    pub fn contract_obj<A>() -> ContractObj<A>
    where
        A: multiversx_sc::api::VMApi,
    {
        ContractObj {
            _phantom: core::marker::PhantomData,
        }
    }
    pub struct ContractBuilder;
    impl multiversx_sc::contract_base::CallableContractBuilder for self::ContractBuilder {
        fn new_contract_obj<A: multiversx_sc::api::VMApi + Send + Sync>(
            &self,
        ) -> multiversx_sc::types::heap::Box<dyn multiversx_sc::contract_base::CallableContract>
        {
            multiversx_sc::types::heap::Box::new(ContractObj::<A> {
                _phantom: core::marker::PhantomData,
            })
        }
    }

    pub struct Proxy<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    impl<A> multiversx_sc::contract_base::ProxyObjBase for Proxy<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        type Api = A;
        type To = ();

        fn extract_opt_address(
            &mut self,
        ) -> multiversx_sc::types::ManagedOption<
            Self::Api,
            multiversx_sc::types::ManagedAddress<Self::Api>,
        > {
            multiversx_sc::types::ManagedOption::none()
        }

        fn extract_address(&mut self) -> multiversx_sc::types::ManagedAddress<Self::Api> {
            multiversx_sc::api::ErrorApiImpl::signal_error(
                &<A as multiversx_sc::api::ErrorApi>::error_api_impl(),
                multiversx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET.as_bytes(),
            )
        }

        fn extract_proxy_to(&mut self) -> Self::To {}
    }

    impl<A> multiversx_sc::contract_base::ProxyObjNew for Proxy<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        type ProxyTo = ProxyTo<A>;

        fn new_proxy_obj() -> Self {
            Proxy {
                _phantom: core::marker::PhantomData,
            }
        }

        fn contract(
            mut self,
            address: multiversx_sc::types::ManagedAddress<Self::Api>,
        ) -> Self::ProxyTo {
            ProxyTo {
                address: multiversx_sc::types::ManagedOption::some(address),
            }
        }
    }

    pub struct ProxyTo<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        pub address:
            multiversx_sc::types::ManagedOption<A, multiversx_sc::types::ManagedAddress<A>>,
    }

    impl<A> multiversx_sc::contract_base::ProxyObjBase for ProxyTo<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        type Api = A;
        type To = multiversx_sc::types::ManagedAddress<A>;

        fn extract_opt_address(
            &mut self,
        ) -> multiversx_sc::types::ManagedOption<
            Self::Api,
            multiversx_sc::types::ManagedAddress<Self::Api>,
        > {
            core::mem::replace(
                &mut self.address,
                multiversx_sc::types::ManagedOption::none(),
            )
        }

        fn extract_address(&mut self) -> multiversx_sc::types::ManagedAddress<Self::Api> {
            let address = core::mem::replace(
                &mut self.address,
                multiversx_sc::types::ManagedOption::none(),
            );
            address.unwrap_or_sc_panic(multiversx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
        }

        fn extract_proxy_to(&mut self) -> Self::To {
            self.extract_address()
        }
    }

    impl<A> super::module_1::ProxyTrait for Proxy<A> where A: multiversx_sc::api::VMApi {}
    impl<A> super::module_1::ProxyTrait for ProxyTo<A> where A: multiversx_sc::api::VMApi {}

    impl<A> ProxyTrait for Proxy<A> where A: multiversx_sc::api::VMApi {}
    impl<A> ProxyTrait for ProxyTo<A> where A: multiversx_sc::api::VMApi {}

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
        fn my_callback(self, caller: &Address) -> multiversx_sc::types::CallbackClosure<Self::Api> {
            let mut ___callback_call___ =
                multiversx_sc::types::new_callback_call::<Self::Api>("my_callback");
            ___callback_call___.push_endpoint_arg(caller);
            ___callback_call___
        }
    }
    impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: multiversx_sc::api::VMApi + 'static {}
}

#[test]
fn contract_without_macros_basic() {
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let adder = sample_adder::contract_obj::<SingleTxApi>();

    adder.init(multiversx_sc::types::BigUint::from(5u32));
    assert_eq!(multiversx_sc::types::BigUint::from(5u32), adder.sum().get());

    adder.add(multiversx_sc::types::BigUint::from(7u32));
    assert_eq!(
        multiversx_sc::types::BigUint::from(12u32),
        adder.sum().get()
    );

    assert_eq!(BigInt::from(100), adder.version());

    let adder = sample_adder::ContractBuilder.new_contract_obj::<SingleTxApi>();
    assert!(!adder.call("invalid_endpoint"));

    let adder = sample_adder::ContractBuilder.new_contract_obj::<SingleTxApi>();
    assert!(adder.call("getSum"));

    let mut own_proxy =
        sample_adder::Proxy::<StaticApi>::new_proxy_obj().contract(ManagedAddress::zero());
    let _ = own_proxy.sum();

    let _ = multiversx_sc_meta_lib::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

fn world() -> multiversx_sc_scenario::ScenarioWorld {
    let mut blockchain = multiversx_sc_scenario::ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("framework/scenario");
    blockchain.register_contract(
        "mxsc:../../contracts/examples/adder/output/adder.mxsc.json",
        sample_adder::ContractBuilder,
    );
    blockchain
}

#[test]
fn contract_without_macros_scenario() {
    world().run("../../contracts/examples/adder/scenarios/adder.scen.json");
}
