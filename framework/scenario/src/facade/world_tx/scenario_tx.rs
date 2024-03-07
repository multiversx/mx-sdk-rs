use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        AnnotatedValue, FunctionCall, ManagedAddress, RHListSync, Tx, TxEnv, TxFromSpecified,
        TxGas, TxPayment, TxToSpecified,
    },
};

use crate::{
    api::StaticApi,
    scenario_model::{AddressValue, ScCallStep},
    ScenarioWorld,
};

use super::{RHListScenario, ScenarioTxEnvironment, TxScenarioBase};

impl ScenarioWorld {
    fn tx_env(&self) -> ScenarioTxEnvironment {
        ScenarioTxEnvironment {
            context_path: self.current_dir.clone(),
            ..Default::default()
        }
    }

    pub fn tx_return<STx, F>(&mut self, f: F) -> STx::Returns
    where
        STx: ScenarioTx,
        F: FnOnce(TxScenarioBase) -> STx,
    {
        let env = self.tx_env();
        let tx_base = TxScenarioBase::new_with_env(env);
        let tx = f(tx_base);
        tx.run_as_scenario_step(self)
    }

    pub fn tx<STx, F>(&mut self, f: F) -> &mut Self
    where
        STx: ScenarioTx<Returns = ()>,
        F: FnOnce(TxScenarioBase) -> STx,
    {
        self.tx_return(f);
        self
    }
}

pub trait ScenarioTx {
    type Returns;

    fn run_as_scenario_step(self, world: &mut ScenarioWorld) -> Self::Returns;
}

fn address_annotated<Env, Addr>(env: &Env, from: Addr) -> AddressValue
where
    Env: TxEnv,
    Addr: AnnotatedValue<Env, ManagedAddress<Env::Api>>,
{
    let annotation = from.annotation(env).to_string();
    AddressValue {
        value: from.into_value(env).to_address(),
        original: ValueSubTree::Str(annotation),
    }
}

impl<From, To, Payment, Gas, RH> ScenarioTx
    for Tx<ScenarioTxEnvironment, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<ScenarioTxEnvironment>,
    To: TxToSpecified<ScenarioTxEnvironment>,
    Payment: TxPayment<ScenarioTxEnvironment>,
    Gas: TxGas<ScenarioTxEnvironment>,
    RH: RHListScenario,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run_as_scenario_step(self, world: &mut ScenarioWorld) -> Self::Returns {
        let mut env = self.env;
        let mut step = ScCallStep::new()
            .from(address_annotated(&env, self.from))
            .to(address_annotated(&env, self.to))
            .function(self.data.function_name.to_string().as_str());
        for arg in self.data.arg_buffer.iter_buffers() {
            step = step.argument(arg.to_vec());
        }

        world.sc_call(&mut step);
        let response = step.response.expect("step did not return result");

        let tuple_result = self.result_handler.item_scenario_result(&response);
        tuple_result.flatten_unpack()
    }
}
