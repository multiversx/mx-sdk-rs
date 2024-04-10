use std::{ops::Add, path::PathBuf};

use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        Address, AddressExpr, AnnotatedValue, Code, DeployCall, FunctionCall, ManagedAddress,
        ManagedBuffer, RHListExec, Tx, TxBaseWithEnv, TxCodeSource, TxCodeSourceSpecified,
        TxCodeValue, TxEnv, TxFromSpecified, TxGas, TxPayment, TxTo, TxToSpecified,
    },
};
use serde_json::map::OccupiedEntry;

use crate::{
    api::StaticApi,
    scenario::{tx_to_step::TxToStep, ScenarioRunner},
    scenario_model::{
        Account, AddressKey, AddressValue, BigUintValue, BytesKey, BytesValue, Esdt, EsdtObject,
        NewAddress, ScCallStep, ScDeployStep, SetStateStep, TxExpect, TxResponse, U64Value,
    },
    ScenarioTxEnv, ScenarioTxRun, ScenarioWorld,
};

use super::ScenarioTxEnvData;

/// Environment for executing transactions.
pub struct ScenarioEnvExec<'w> {
    pub world: &'w mut ScenarioWorld,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for ScenarioEnvExec<'w> {
    type Api = StaticApi;

    type RHExpect = TxExpect;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas_annotation(&self) -> ManagedBuffer<Self::Api> {
        self.data.default_gas_annotation()
    }

    fn default_gas_value(&self) -> u64 {
        self.data.default_gas_value()
    }
}

impl<'w> ScenarioTxEnv for ScenarioEnvExec<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

impl<'w, From, To, Payment, Gas, RH> ScenarioTxRun
    for Tx<ScenarioEnvExec<'w>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    To: TxToSpecified<ScenarioEnvExec<'w>>,
    Payment: TxPayment<ScenarioEnvExec<'w>>,
    Gas: TxGas<ScenarioEnvExec<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step_wrapper = self.tx_to_step();
        step_wrapper.env.world.sc_call(&mut step_wrapper.step);
        step_wrapper.process_result()
    }
}

impl ScenarioWorld {
    pub fn tx(&mut self) -> TxBaseWithEnv<ScenarioEnvExec<'_>> {
        let data = self.new_env_data();
        let env = ScenarioEnvExec { world: self, data };
        Tx::new_with_env(env)
    }

    pub fn chain_call<From, To, Payment, Gas, RH, F>(&mut self, f: F) -> &mut Self
    where
        From: TxFromSpecified<ScenarioTxEnvData>,
        To: TxToSpecified<ScenarioTxEnvData>,
        Payment: TxPayment<ScenarioTxEnvData>,
        Gas: TxGas<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        )
            -> Tx<ScenarioTxEnvData, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        let mut step_wrapper = tx.tx_to_step();
        self.sc_call(&mut step_wrapper.step);
        step_wrapper.process_result();
        self
    }

    pub fn set_state(&mut self) -> SetStateBuilder<'_> {
        SetStateBuilder::new(self)
    }
}

pub struct SetStateBuilder<'w> {
    world: &'w mut ScenarioWorld,
    set_state_step: SetStateStep,
    address_expr: AddressKey,
}

impl<'w> SetStateBuilder<'w> {
    pub fn new(world: &'w mut ScenarioWorld) -> SetStateBuilder<'w> {
        SetStateBuilder {
            world,
            set_state_step: SetStateStep::new(),
            address_expr: AddressKey::default(),
        }
    }

    pub fn account<A>(mut self, address_expr: A) -> Self
    where
        AddressKey: From<A>,
    {
        let account = Account::new();
        let address_key = AddressKey::from(address_expr);

        self.address_expr = address_key.clone();
        self.set_state_step.accounts.insert(address_key, account);
        self
    }

    pub fn nonce<V>(mut self, nonce: V) -> Self
    where
        U64Value: From<V>,
    {
        self.set_state_step
            .accounts
            .entry(self.address_expr.clone())
            .and_modify(|entry| {
                entry.nonce = Some(U64Value::from(nonce));
            });

        self
    }

    pub fn balance<V>(mut self, balance_expr: V) -> Self
    where
        BigUintValue: From<V>,
    {
        self.set_state_step
            .accounts
            .entry(self.address_expr.clone())
            .and_modify(|entry| {
                entry.balance = Some(BigUintValue::from(balance_expr));
            });
        self
    }

    pub fn esdt_balance<K, V>(mut self, token_id_expr: K, balance_expr: V) -> Self
    where
        BytesKey: From<K>,
        BigUintValue: From<V>,
    {
        let token_id = BytesKey::from(token_id_expr);
        let esdt_data_ref = self.get_esdt_data_or_create(&token_id);
        esdt_data_ref.set_balance(0u64, balance_expr);

        self
    }

    pub fn esdt_nft_balance<K, N, V, T>(
        mut self,
        token_id_expr: K,
        nonce_expr: N,
        balance_expr: V,
        opt_attributes_expr: Option<T>,
    ) -> Self
    where
        N: Clone,
        BytesKey: From<K>,
        U64Value: From<N>,
        BigUintValue: From<V>,
        BytesValue: From<T>,
    {
        let token_id = BytesKey::from(token_id_expr);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id)
            .get_mut_esdt_object();
        esdt_obj_ref.set_balance(nonce_expr.clone(), balance_expr);

        if let Some(attributes_expr) = opt_attributes_expr {
            esdt_obj_ref.set_token_attributes(nonce_expr, attributes_expr);
        }

        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn esdt_nft_all_properties<K, N, V, T>(
        mut self,
        token_id_expr: K,
        nonce_expr: N,
        balance_expr: V,
        opt_attributes_expr: Option<T>,
        royalties_expr: N,
        creator_expr: Option<T>,
        hash_expr: Option<T>,
        uris_expr: Vec<T>,
    ) -> Self
    where
        BytesKey: From<K>,
        U64Value: From<N>,
        BigUintValue: From<V>,
        BytesValue: From<T>,
    {
        let token_id = BytesKey::from(token_id_expr);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id)
            .get_mut_esdt_object();

        esdt_obj_ref.set_token_all_properties(
            nonce_expr,
            balance_expr,
            opt_attributes_expr,
            royalties_expr,
            creator_expr,
            hash_expr,
            uris_expr,
        );

        self
    }

    pub fn esdt_nft_last_nonce<K, N>(mut self, token_id_expr: K, last_nonce_expr: N) -> Self
    where
        BytesKey: From<K>,
        U64Value: From<N>,
    {
        let token_id = BytesKey::from(token_id_expr);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id)
            .get_mut_esdt_object();
        esdt_obj_ref.set_last_nonce(last_nonce_expr);

        self
    }

    // TODO: Find a better way to pass roles
    pub fn esdt_roles<K>(mut self, token_id_expr: K, roles: Vec<String>) -> Self
    where
        BytesKey: From<K>,
    {
        let token_id = BytesKey::from(token_id_expr);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id)
            .get_mut_esdt_object();
        esdt_obj_ref.set_roles(roles);

        self
    }

    fn get_esdt_data_or_create(&mut self, token_id: &BytesKey) -> &mut Esdt {
        let entry = self
            .set_state_step
            .accounts
            .entry(self.address_expr.clone())
            .or_default();

        if !entry.esdt.contains_key(token_id) {
            let _ = entry
                .esdt
                .insert(token_id.clone(), Esdt::Full(EsdtObject::default()));
        }

        entry.esdt.get_mut(token_id).unwrap()
    }

    pub fn code<V>(mut self, code_expr: V) -> Self
    where
        BytesValue: From<V>,
    {
        self.set_state_step
            .accounts
            .entry(self.address_expr.clone())
            .and_modify(|entry| {
                entry.code = Some(BytesValue::from(code_expr));
            });

        self
    }

    pub fn owner<V>(mut self, owner_expr: V) -> Self
    where
        AddressValue: From<V>,
    {
        self.set_state_step
            .accounts
            .entry(self.address_expr.clone())
            .and_modify(|entry| {
                entry.owner = Some(AddressValue::from(owner_expr));
            });

        self
    }

    pub fn new_address<CA, NA>(
        mut self,
        creator_address_expr: CA,
        creator_nonce_expr: u64,
        new_address_expr: NA,
    ) -> Self
    where
        AddressValue: From<CA>,
        AddressValue: From<NA>,
    {
        self.set_state_step.new_addresses.push(NewAddress {
            creator_address: AddressValue::from(creator_address_expr),
            creator_nonce: U64Value::from(creator_nonce_expr),
            new_address: AddressValue::from(new_address_expr),
        });

        self
    }

    pub fn commit(self) {
        self.world.run_set_state_step(&self.set_state_step);
    }
}
