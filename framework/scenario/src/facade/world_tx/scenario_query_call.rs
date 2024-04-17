use std::{
    collections::{btree_map::Entry, BTreeMap},
    path::PathBuf,
};

use multiversx_chain_scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext};
use multiversx_sc::{
    codec::test_util::top_encode_to_vec_u8_or_panic,
    proxy_imports::TopEncode,
    tuple_util::NestedTupleFlatten,
    types::{
        AnnotatedValue, FunctionCall, ManagedAddress, ManagedBuffer, RHListExec, Tx, TxBaseWithEnv,
        TxEnv, TxFromSpecified, TxGas, TxPayment, TxToSpecified,
    },
};

use crate::{
    api::StaticApi,
    scenario::{tx_to_step::TxToQueryStep, ScenarioRunner},
    scenario_model::{
        AddressKey, BigUintValue, BytesKey, BytesValue, CheckAccount, CheckEsdt, CheckEsdtData,
        CheckEsdtInstances, CheckEsdtMap, CheckEsdtMapContents, CheckStateStep, CheckStorage,
        CheckStorageDetails, CheckValue, TxExpect, TxResponse, U64Value,
    },
    ScenarioTxEnv, ScenarioTxEnvData, ScenarioTxRun, ScenarioWorld,
};

pub struct ScenarioEnvQuery<'w> {
    pub world: &'w mut ScenarioWorld,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for ScenarioEnvQuery<'w> {
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

impl<'w> ScenarioTxEnv for ScenarioEnvQuery<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

impl<'w, To, RH> ScenarioTxRun
    for Tx<ScenarioEnvQuery<'w>, (), To, (), (), FunctionCall<StaticApi>, RH>
where
    To: TxToSpecified<ScenarioEnvQuery<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step_wrapper = self.tx_to_query_step();
        step_wrapper.env.world.sc_query(&mut step_wrapper.step);
        step_wrapper.process_result()
    }
}

impl ScenarioWorld {
    pub fn query(&mut self) -> TxBaseWithEnv<ScenarioEnvQuery<'_>> {
        let data = self.new_env_data();
        let env = ScenarioEnvQuery { world: self, data };
        Tx::new_with_env(env)
    }

    pub fn chain_query<To, RH, F>(&mut self, f: F) -> &mut Self
    where
        To: TxToSpecified<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        ) -> Tx<ScenarioTxEnvData, (), To, (), (), FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        let mut step_wrapper = tx.tx_to_query_step();
        self.sc_query(&mut step_wrapper.step);
        step_wrapper.process_result();
        self
    }

    pub fn check_state_account<A>(&mut self, address: A) -> CheckStateBuilder<'_>
    where
        AddressKey: From<A>,
    {
        CheckStateBuilder::new(self, address.into())
    }
}

pub struct CheckStateBuilder<'w> {
    world: &'w mut ScenarioWorld,
    check_state_step: CheckStateStep,
    current_account: CheckAccount,
    current_address: AddressKey,
}

impl<'w> CheckStateBuilder<'w> {
    pub(crate) fn new(world: &'w mut ScenarioWorld, address: AddressKey) -> CheckStateBuilder<'w> {
        let mut builder = CheckStateBuilder {
            world,
            check_state_step: CheckStateStep::new(),
            current_account: CheckAccount::new(),
            current_address: AddressKey::default(),
        };
        builder.reset_account(address);
        builder
    }

    /// Starts building of a new account.
    pub fn check_state_account<A>(mut self, address_expr: A) -> Self
    where
        AddressKey: From<A>,
    {
        self.add_current_acount();
        self.reset_account(address_expr.into());
        self
    }

    fn add_current_acount(&mut self) {
        if let Entry::Vacant(entry) = self
            .check_state_step
            .accounts
            .accounts
            .entry(core::mem::take(&mut self.current_address))
        {
            entry.insert(core::mem::take(&mut self.current_account));
        };
    }

    fn reset_account(&mut self, address: AddressKey) {
        self.current_address = address;
        self.current_account = CheckAccount::default();
    }

    /// Finished and sets all account in the blockchain mock.
    fn commit_accounts(&mut self) {
        self.add_current_acount();
        self.world.run_check_state_step(&self.check_state_step);
    }

    /// Forces value drop and commit accounts.
    pub fn commit(self) {}

    pub fn nonce<V>(mut self, nonce: V) -> Self
    where
        U64Value: InterpretableFrom<V>,
    {
        self.current_account.nonce = CheckValue::Equal(U64Value::interpret_from(
            nonce,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn balance<V>(mut self, balance_expr: V) -> Self
    where
        BigUintValue: InterpretableFrom<V>,
    {
        self.current_account.balance = CheckValue::Equal(BigUintValue::interpret_from(
            balance_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn code<V>(mut self, code_expr: V) -> Self
    where
        BytesValue: InterpretableFrom<V>,
    {
        self.current_account.code = CheckValue::Equal(BytesValue::interpret_from(
            code_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn code_metadata<V>(mut self, code_metadata_expr: V) -> Self
    where
        BytesValue: InterpretableFrom<V>,
    {
        self.current_account.code_metadata = CheckValue::Equal(BytesValue::interpret_from(
            code_metadata_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn esdt_balance<K, V>(mut self, token_id_expr: K, balance_expr: V) -> Self
    where
        BytesKey: From<K>,
        BigUintValue: From<V>,
    {
        let token_id = BytesKey::from(token_id_expr);
        let balance = BigUintValue::from(balance_expr);

        match &mut self.current_account.esdt {
            CheckEsdtMap::Unspecified | CheckEsdtMap::Star => {
                let mut new_esdt_map = BTreeMap::new();
                let _ = new_esdt_map.insert(token_id, CheckEsdt::Short(balance));

                let new_check_esdt_map = CheckEsdtMapContents {
                    contents: new_esdt_map,
                    other_esdts_allowed: true,
                };

                self.current_account.esdt = CheckEsdtMap::Equal(new_check_esdt_map);
            },
            CheckEsdtMap::Equal(check_esdt_map) => {
                if check_esdt_map.contents.contains_key(&token_id) {
                    let prev_entry = check_esdt_map.contents.get_mut(&token_id).unwrap();
                    match prev_entry {
                        CheckEsdt::Short(prev_balance_check) => *prev_balance_check = balance,
                        CheckEsdt::Full(prev_esdt_check) => match prev_esdt_check.instances {
                            CheckEsdtInstances::Star => todo!(),
                            CheckEsdtInstances::Equal(_) => todo!(),
                        },
                    }
                }
            },
        }

        self
    }

    pub fn esdt_nft_balance_and_attributes<K, N, V, T>(
        mut self,
        token_id_expr: K,
        nonce_expr: N,
        balance_expr: V,
        attributes_expr: Option<T>,
    ) -> Self
    where
        BytesKey: From<K>,
        U64Value: From<N>,
        BigUintValue: From<V>,
        T: TopEncode,
    {
        let token_id = BytesKey::from(token_id_expr);

        if let CheckEsdtMap::Unspecified = &self.current_account.esdt {
            let mut check_esdt = CheckEsdt::Full(CheckEsdtData::default());

            if let Some(attributes_expr) = attributes_expr {
                check_esdt.add_balance_and_attributes_check(
                    nonce_expr,
                    balance_expr,
                    top_encode_to_vec_u8_or_panic(&attributes_expr),
                );
            } else {
                check_esdt.add_balance_and_attributes_check(
                    nonce_expr,
                    balance_expr,
                    Vec::<u8>::new(),
                );
            }

            let mut new_esdt_map = BTreeMap::new();
            let _ = new_esdt_map.insert(token_id, check_esdt);

            let new_check_esdt_map = CheckEsdtMapContents {
                contents: new_esdt_map,
                other_esdts_allowed: true,
            };

            self.current_account.esdt = CheckEsdtMap::Equal(new_check_esdt_map);
        }

        self
    }

    pub fn check_storage(mut self, key: &str, value: &str) -> Self {
        let mut details = match &self.current_account.storage {
            CheckStorage::Star => CheckStorageDetails::default(),
            CheckStorage::Equal(details) => details.clone(),
        };
        details.storages.insert(
            BytesKey::interpret_from(key, &InterpreterContext::default()),
            CheckValue::Equal(BytesValue::interpret_from(
                value,
                &InterpreterContext::default(),
            )),
        );
        self.current_account.storage = CheckStorage::Equal(details);
        self
    }
}

impl<'w> Drop for CheckStateBuilder<'w> {
    fn drop(&mut self) {
        self.commit_accounts();
    }
}
