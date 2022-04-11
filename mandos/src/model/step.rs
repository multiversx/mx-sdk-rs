use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::StepRaw,
};

use std::collections::BTreeMap;

use super::{
    Account, AddressKey, BlockInfo, BytesValue, CheckAccounts, NewAddress, TxCall, TxDeploy,
    TxExpect, TxQuery, TxTransfer, TxValidatorReward,
};

#[derive(Debug)]
pub struct ExternalStepsStep {
    pub comment: Option<String>,
    pub path: String,
}

#[derive(Debug, Default)]
pub struct SetStateStep {
    pub comment: Option<String>,
    pub accounts: BTreeMap<AddressKey, Account>,
    pub new_addresses: Vec<NewAddress>,
    pub block_hashes: Vec<BytesValue>,
    pub previous_block_info: Box<Option<BlockInfo>>,
    pub current_block_info: Box<Option<BlockInfo>>,
}

#[derive(Debug, Default)]
pub struct ScCallStep {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxCall>,
    pub expect: Option<TxExpect>,
}

#[derive(Debug, Default)]
pub struct ScQueryStep {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxQuery>,
    pub expect: Option<TxExpect>,
}

#[derive(Debug, Default)]
pub struct ScDeployStep {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxDeploy>,
    pub expect: Option<TxExpect>,
}

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

#[derive(Debug, Default)]
pub struct CheckStateStep {
    pub comment: Option<String>,
    pub accounts: CheckAccounts,
}

#[derive(Debug, Default)]
pub struct DumpStateStep {
    pub comment: Option<String>,
}

#[derive(Debug)]
pub enum Step {
    ExternalSteps(ExternalStepsStep),
    SetState(SetStateStep),
    ScCall(ScCallStep),
    ScQuery(ScQueryStep),
    ScDeploy(ScDeployStep),
    Transfer(TransferStep),
    ValidatorReward(ValidatorRewardStep),
    CheckState(CheckStateStep),
    DumpState(DumpStateStep),
}

impl InterpretableFrom<StepRaw> for Step {
    fn interpret_from(from: StepRaw, context: &InterpreterContext) -> Self {
        match from {
            StepRaw::ExternalSteps { comment, path } => {
                Step::ExternalSteps(ExternalStepsStep { comment, path })
            },
            StepRaw::SetState {
                comment,
                accounts,
                new_addresses,
                block_hashes,
                previous_block_info,
                current_block_info,
            } => Step::SetState(SetStateStep {
                comment,
                accounts: accounts
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            AddressKey::interpret_from(k, context),
                            Account::interpret_from(v, context),
                        )
                    })
                    .collect(),
                new_addresses: new_addresses
                    .into_iter()
                    .map(|t| NewAddress::interpret_from(t, context))
                    .collect(),
                block_hashes: block_hashes
                    .into_iter()
                    .map(|t| BytesValue::interpret_from(t, context))
                    .collect(),
                previous_block_info: Box::new(
                    previous_block_info.map(|v| BlockInfo::interpret_from(v, context)),
                ),
                current_block_info: Box::new(
                    current_block_info.map(|v| BlockInfo::interpret_from(v, context)),
                ),
            }),
            StepRaw::ScCall {
                tx_id,
                comment,
                display_logs: _,
                tx,
                expect,
            } => Step::ScCall(ScCallStep {
                tx_id,
                comment,
                tx: Box::new(TxCall::interpret_from(tx, context)),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            }),
            StepRaw::ScQuery {
                tx_id,
                comment,
                display_logs: _,
                tx,
                expect,
            } => Step::ScQuery(ScQueryStep {
                tx_id,
                comment,
                tx: Box::new(TxQuery::interpret_from(tx, context)),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            }),
            StepRaw::ScDeploy {
                tx_id,
                comment,
                display_logs: _,
                tx,
                expect,
            } => Step::ScDeploy(ScDeployStep {
                tx_id,
                comment,
                tx: Box::new(TxDeploy::interpret_from(tx, context)),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            }),
            StepRaw::Transfer { tx_id, comment, tx } => Step::Transfer(TransferStep {
                tx_id,
                comment,
                tx: Box::new(TxTransfer::interpret_from(tx, context)),
            }),
            StepRaw::ValidatorReward { tx_id, comment, tx } => {
                Step::ValidatorReward(ValidatorRewardStep {
                    tx_id,
                    comment,
                    tx: Box::new(TxValidatorReward::interpret_from(tx, context)),
                })
            },
            StepRaw::CheckState { comment, accounts } => Step::CheckState(CheckStateStep {
                comment,
                accounts: CheckAccounts::interpret_from(accounts, context),
            }),
            StepRaw::DumpState { comment } => Step::DumpState(DumpStateStep { comment }),
        }
    }
}

impl IntoRaw<StepRaw> for Step {
    fn into_raw(self) -> StepRaw {
        match self {
            Step::ExternalSteps(s) => StepRaw::ExternalSteps {
                comment: s.comment,
                path: s.path,
            },
            Step::SetState(s) => StepRaw::SetState {
                comment: s.comment,
                accounts: s
                    .accounts
                    .into_iter()
                    .map(|(address, account)| (address.into_raw(), account.into_raw()))
                    .collect(),
                new_addresses: s
                    .new_addresses
                    .into_iter()
                    .map(|na| na.into_raw())
                    .collect(),
                block_hashes: s.block_hashes.into_iter().map(|bh| bh.original).collect(),
                previous_block_info: s.previous_block_info.map(|bi| bi.into_raw()),
                current_block_info: s.current_block_info.map(|bi| bi.into_raw()),
            },
            Step::ScCall(s) => StepRaw::ScCall {
                tx_id: s.tx_id,
                comment: s.comment,
                display_logs: None,
                tx: s.tx.into_raw(),
                expect: s.expect.map(|expect| expect.into_raw()),
            },
            Step::ScQuery(s) => StepRaw::ScQuery {
                tx_id: s.tx_id,
                comment: s.comment,
                display_logs: None,
                tx: s.tx.into_raw(),
                expect: s.expect.map(|expect| expect.into_raw()),
            },
            Step::ScDeploy(s) => StepRaw::ScDeploy {
                tx_id: s.tx_id,
                comment: s.comment,
                display_logs: None,
                tx: s.tx.into_raw(),
                expect: s.expect.map(|expect| expect.into_raw()),
            },
            Step::Transfer(s) => StepRaw::Transfer {
                tx_id: s.tx_id,
                comment: s.comment,
                tx: s.tx.into_raw(),
            },
            Step::ValidatorReward(s) => StepRaw::ValidatorReward {
                tx_id: s.tx_id,
                comment: s.comment,
                tx: s.tx.into_raw(),
            },
            Step::CheckState(s) => StepRaw::CheckState {
                comment: s.comment,
                accounts: s.accounts.into_raw(),
            },
            Step::DumpState(s) => StepRaw::DumpState { comment: s.comment },
        }
    }
}
