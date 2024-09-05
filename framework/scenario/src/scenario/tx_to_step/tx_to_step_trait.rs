use super::StepWrapper;

pub trait TxToStep<Env, RH> {
    type Step;

    fn tx_to_step(self) -> StepWrapper<Env, Self::Step, RH>;
}

pub trait TxToQueryStep<Env, RH> {
    type Step;

    fn tx_to_query_step(self) -> StepWrapper<Env, Self::Step, RH>;
}
