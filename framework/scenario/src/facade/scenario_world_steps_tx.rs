use std::path::PathBuf;

use multiversx_sc::types::{
    AnnotatedValue, FunctionCall, ManagedAddress, Tx, TxBaseWithEnv, TxEnvironemnt,
    TxFromSpecified, TxGas, TxPayment, TxToSpecified,
};

use crate::{api::StaticApi, facade::ScenarioWorld, scenario_model::ScCallStep};

#[derive(Default, Debug, Clone)]
pub struct ScenarioTxEnvironment {
    pub context_path: PathBuf,
    pub from_annotation: Option<String>,
    pub to_annotation: Option<String>,
}

impl TxEnvironemnt<StaticApi> for ScenarioTxEnvironment {
    fn annotate_from<From>(&mut self, to: &From)
    where
        From: AnnotatedValue<StaticApi, ManagedAddress<StaticApi>>,
    {
        self.from_annotation = Some(to.annotation().to_string())
    }

    fn annotate_to<To>(&mut self, to: &To)
    where
        To: AnnotatedValue<StaticApi, ManagedAddress<StaticApi>>,
    {
        self.to_annotation = Some(to.annotation().to_string())
    }
}

pub type TxScenarioBase = TxBaseWithEnv<StaticApi, ScenarioTxEnvironment>;

pub trait ScenarioTx {
    fn run_as_scenario_step(self, world: &mut ScenarioWorld);
}

impl ScenarioWorld {
    fn tx_env(&self) -> ScenarioTxEnvironment {
        ScenarioTxEnvironment {
            context_path: self.current_dir.clone(),
            ..Default::default()
        }
    }

    pub fn tx<STx, F>(&mut self, f: F) -> &mut Self
    where
        STx: ScenarioTx,
        F: FnOnce(TxScenarioBase) -> STx,
    {
        let env = self.tx_env();
        let tx_base = TxScenarioBase::new_with_env(env);
        let tx = f(tx_base);
        tx.run_as_scenario_step(self);
        self
    }
}

impl<From, To, Payment, Gas> ScenarioTx
    for Tx<StaticApi, ScenarioTxEnvironment, From, To, Payment, Gas, FunctionCall<StaticApi>>
where
    From: TxFromSpecified<StaticApi>,
    To: TxToSpecified<StaticApi>,
    Payment: TxPayment<StaticApi>,
    Gas: TxGas,
{
    fn run_as_scenario_step(self, world: &mut ScenarioWorld) {
        let mut step = ScCallStep::new()
            .from(self.env.from_annotation.unwrap().as_str())
            .to(self.env.to_annotation.unwrap().as_str())
            .function(self.data.function_name.to_string().as_str());
        for arg in self.data.arg_buffer.iter_buffers() {
            step = step.argument(arg.to_vec());
        }

        world.sc_call(step);
    }
}
