use crate::mandos_system::model::{TxTransfer, TxValidatorReward};

#[derive(Debug)]
pub struct TransferStep {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxTransfer>,
}

#[derive(Debug)]
pub struct ValidatorRewardStep {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxValidatorReward>,
}
