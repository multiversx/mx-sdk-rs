use super::StepWrapper;

pub trait TxToStep {
    type Env;
    type Step;
    type RH;

    fn tx_to_step(self) -> StepWrapper<Self::Env, Self::Step, Self::RH>;
}

pub trait TxToQueryStep {
    type Env;
    type Step;
    type RH;

    fn tx_to_query_step(self) -> StepWrapper<Self::Env, Self::Step, Self::RH>;
}
