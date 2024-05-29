// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

#![allow(unused)]

use multiversx_sc::{
    contract_base::ProxyObjNew,
    types::{BigInt, BigUint, ManagedAddress},
};
use multiversx_sc_scenario::api::{SingleTxApi, StaticApi};

use crate::module_1::VersionModule;

mod module_1 {
    multiversx_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: ContractBase + Sized {
        fn version(&self) -> BigInt<Self::Api>;

        fn some_async(&self);

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: ContractBase {}

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

    pub trait EndpointWrappers: VersionModule + ContractBase {
        #[inline]
        fn call_version(&self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
            let result = self.version();
            multiversx_sc::io::finish_multi::<Self::Api, _>(&result);
        }

        fn call_some_async(&self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
            self.some_async();
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

    pub trait ProxyTrait: ProxyObjBase + Sized {
        fn version(
            &mut self,
        ) -> Tx<
            TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            FunctionCall<Self::Api>,
            OriginalResultMarker<BigInt<Self::Api>>,
        > {
            TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("version")
        }
    }
}

mod sample_adder {
    use multiversx_sc::storage::StorageKey;

    multiversx_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait Adder: super::module_1::VersionModule + ContractBase + Sized {
        fn init(&self, initial_value: BigUint<Self::Api>) {
            self.sum().set(initial_value);
        }
        fn upgrade(&self, initial_value: BigUint<Self::Api>) {
            self.init(initial_value);
        }
        fn add(&self, value: BigUint<Self::Api>) {
            self.sum().update(|sum| *sum += value);
        }
        fn sum(&self) -> SingleValueMapper<Self::Api, BigUint<Self::Api>>;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: ContractBase {}

    // impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        fn sum(&self) -> SingleValueMapper<Self::Api, BigUint<Self::Api>> {
            let mut ___key___ = StorageKey::<Self::Api>::new(&b"sum"[..]);
            <SingleValueMapper<Self::Api, BigUint<Self::Api>> as StorageMapper<Self::Api>>::new(
                ___key___,
            )
        }
    }

    impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A> where
        A: multiversx_sc::api::VMApi
    {
    }

    pub trait EndpointWrappers: Adder + ContractBase + super::module_1::EndpointWrappers {
        #[inline]
        fn call_sum(&self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let () = multiversx_sc::io::load_endpoint_args::<Self::Api, ()>(());
            let result = self.sum();
            multiversx_sc::io::finish_multi::<Self::Api, _>(&result);
        }
        #[inline]
        fn call_init(&self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let (initial_value, ()) = multiversx_sc::io::load_endpoint_args::<
                Self::Api,
                (BigUint<Self::Api>, ()),
            >(("initial_value", ()));
            self.init(initial_value);
        }
        #[inline]
        fn call_upgrade(&self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let (initial_value, ()) = multiversx_sc::io::load_endpoint_args::<
                Self::Api,
                (BigUint<Self::Api>, ()),
            >(("initial_value", ()));
            self.upgrade(initial_value);
        }
        #[inline]
        fn call_add(&self) {
            <Self::Api as multiversx_sc::api::VMApi>::init_static();
            multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            let (value, ()) = multiversx_sc::io::load_endpoint_args::<
                Self::Api,
                (BigUint<Self::Api>, ()),
            >(("value", ()));
            self.add(value);
        }

        fn call(&self, fn_name: &str) -> bool {
            if match fn_name {
                "callBack" => {
                    EndpointWrappers::callback(self);
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
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector(
            &self,
            mut ___cb_closure___: CallbackClosureForDeser<Self::Api>,
        ) -> CallbackSelectorResult<Self::Api> {
            CallbackSelectorResult::NotProcessed(___cb_closure___)
        }
        fn callback(&self) {}
    }

    impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A> where
        A: multiversx_sc::api::VMApi
    {
    }

    pub trait ProxyTrait: ProxyObjBase + super::module_1::ProxyTrait {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn sum(
            &mut self,
        ) -> Tx<
            TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            FunctionCall<Self::Api>,
            OriginalResultMarker<SingleValueMapper<Self::Api, BigUint<Self::Api>>>,
        > {
            TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("getSum")
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn init<Arg0: ProxyArg<BigUint<Self::Api>>>(
            &mut self,
            initial_value: Arg0,
        ) -> Tx<
            TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            DeployCall<TxScEnv<Self::Api>, ()>,
            OriginalResultMarker<()>,
        > {
            TxBaseWithEnv::new_tx_from_sc()
                .raw_deploy()
                .argument(&initial_value)
                .original_result()
                .to(self.extract_proxy_to())
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn upgrade<Arg0: ProxyArg<BigUint<Self::Api>>>(
            &mut self,
            initial_value: Arg0,
        ) -> Tx<
            TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            FunctionCall<Self::Api>,
            OriginalResultMarker<()>,
        > {
            TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("upgrade")
                .argument(&initial_value)
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn add<Arg0: ProxyArg<BigUint<Self::Api>>>(
            &mut self,
            value: Arg0,
        ) -> Tx<
            TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            FunctionCall<Self::Api>,
            OriginalResultMarker<()>,
        > {
            TxBaseWithEnv::new_tx_from_sc()
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
    impl<A> ContractBase for ContractObj<A>
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
            EndpointWrappers::call(self, fn_name)
        }
    }

    pub struct ContractBuilder;

    impl multiversx_sc::contract_base::CallableContractBuilder for ContractBuilder {
        fn new_contract_obj<A: multiversx_sc::api::VMApi + Send + Sync>(
            &self,
        ) -> Box<dyn multiversx_sc::contract_base::CallableContract> {
            Box::new(ContractObj::<A> {
                _phantom: core::marker::PhantomData,
            })
        }
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
                &[],
                "getSum",
                "sum",
                false,
                false,
                multiversx_sc::abi::EndpointMutabilityAbi::Readonly,
                multiversx_sc::abi::EndpointTypeAbi::Endpoint,
                &[],
                &[],
                false,
            );
            endpoint_abi.add_output::<SingleValueMapper<Self::Api, BigUint<Self::Api>>>(&[]);
            contract_abi
                .add_type_descriptions::<SingleValueMapper<Self::Api, BigUint<Self::Api>>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
                &[],
                "init",
                "init",
                false,
                false,
                multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                multiversx_sc::abi::EndpointTypeAbi::Init,
                &[],
                &[],
                false,
            );
            endpoint_abi.add_input::<BigUint<Self::Api>>("initial_value");
            contract_abi.add_type_descriptions::<BigUint<Self::Api>>();
            contract_abi.constructors.push(endpoint_abi);
            let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
                &[],
                "upgrade",
                "upgrade",
                false,
                false,
                multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                multiversx_sc::abi::EndpointTypeAbi::Upgrade,
                &[],
                &[],
                false,
            );
            endpoint_abi.add_input::<BigUint<Self::Api>>("initial_value");
            contract_abi.add_type_descriptions::<BigUint<Self::Api>>();
            contract_abi.upgrade_constructors.push(endpoint_abi);
            let mut endpoint_abi = multiversx_sc::abi::EndpointAbi::new(
                &["Add desired amount to the storage variable."],
                "add",
                "add",
                false,
                false,
                multiversx_sc::abi::EndpointMutabilityAbi::Mutable,
                multiversx_sc::abi::EndpointTypeAbi::Endpoint,
                &[],
                &[],
                false,
            );
            endpoint_abi.add_input::<BigUint<Self::Api>>("value");
            contract_abi.add_type_descriptions::<BigUint<Self::Api>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
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

    pub struct Proxy<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    impl<A> ProxyObjBase for Proxy<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        type Api = A;
        type To = ();

        fn extract_opt_address(&mut self) -> ManagedOption<Self::Api, ManagedAddress<Self::Api>> {
            ManagedOption::none()
        }

        fn extract_address(&mut self) -> ManagedAddress<Self::Api> {
            multiversx_sc::api::ErrorApiImpl::signal_error(
                &<A as multiversx_sc::api::ErrorApi>::error_api_impl(),
                multiversx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET.as_bytes(),
            )
        }
        fn extract_proxy_to(&mut self) -> Self::To {}
    }

    impl<A> ProxyObjNew for Proxy<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        type ProxyTo = ProxyTo<A>;
        fn new_proxy_obj() -> Self {
            Proxy {
                _phantom: core::marker::PhantomData,
            }
        }

        fn contract(mut self, address: ManagedAddress<Self::Api>) -> Self::ProxyTo {
            ProxyTo {
                address: ManagedOption::some(address),
            }
        }
    }
    pub struct ProxyTo<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        pub address: ManagedOption<A, ManagedAddress<A>>,
    }

    impl<A> ProxyObjBase for ProxyTo<A>
    where
        A: multiversx_sc::api::VMApi + 'static,
    {
        type Api = A;
        type To = ManagedAddress<A>;
        fn extract_opt_address(&mut self) -> ManagedOption<Self::Api, ManagedAddress<Self::Api>> {
            core::mem::replace(&mut self.address, ManagedOption::none())
        }
        fn extract_address(&mut self) -> ManagedAddress<Self::Api> {
            let address = core::mem::replace(&mut self.address, ManagedOption::none());
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
}

#[test]
fn contract_without_macros_basic() {
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let adder = sample_adder::contract_obj::<SingleTxApi>();

    adder.init(BigUint::from(5u32));
    assert_eq!(BigUint::from(5u32), adder.sum().get());

    adder.add(BigUint::from(7u32));
    assert_eq!(BigUint::from(12u32), adder.sum().get());

    assert_eq!(BigInt::from(100), adder.version());

    assert!(!adder.call("invalid_endpoint"));

    assert!(adder.call("getSum"));

    let mut own_proxy =
        sample_adder::Proxy::<StaticApi>::new_proxy_obj().contract(ManagedAddress::zero());
    let _ = own_proxy.sum();

    let _ = multiversx_sc_meta::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

fn world() -> multiversx_sc_scenario::ScenarioWorld {
    let mut blockchain = multiversx_sc_scenario::ScenarioWorld::new();
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
