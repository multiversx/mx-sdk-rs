use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::StepRaw,
};

use std::collections::BTreeMap;

use super::{
    Account, AddressKey, BlockInfo, BytesValue, CheckAccounts, NewAddress, TxCall, TxDeploy,
    TxExpect, TxQuery, TxTransfer, TxValidatorReward,
};

#[derive(Debug)]
pub enum Step {
    ExternalSteps {
        path: String,
    },
    SetState {
        comment: Option<String>,
        accounts: BTreeMap<AddressKey, Account>,
        new_addresses: Vec<NewAddress>,
        block_hashes: Vec<BytesValue>,
        previous_block_info: Box<Option<BlockInfo>>,
        current_block_info: Box<Option<BlockInfo>>,
    },
    ScCall {
        tx_id: String,
        comment: Option<String>,
        tx: Box<TxCall>,
        expect: Option<TxExpect>,
    },
    ScQuery {
        tx_id: String,
        comment: Option<String>,
        tx: Box<TxQuery>,
        expect: Option<TxExpect>,
    },
    ScDeploy {
        tx_id: String,
        comment: Option<String>,
        tx: Box<TxDeploy>,
        expect: Option<TxExpect>,
    },
    Transfer {
        tx_id: String,
        comment: Option<String>,
        tx: Box<TxTransfer>,
    },
    ValidatorReward {
        tx_id: String,
        comment: Option<String>,
        tx: Box<TxValidatorReward>,
    },
    CheckState {
        comment: Option<String>,
        accounts: CheckAccounts,
    },
    DumpState {
        comment: Option<String>,
    },
}

impl InterpretableFrom<StepRaw> for Step {
    fn interpret_from(from: StepRaw, context: &InterpreterContext) -> Self {
        match from {
            StepRaw::ExternalSteps { comment: _, path } => Step::ExternalSteps { path },
            StepRaw::SetState {
                comment,
                accounts,
                new_addresses,
                block_hashes,
                previous_block_info,
                current_block_info,
            } => Step::SetState {
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
            },
            StepRaw::ScCall {
                tx_id,
                comment,
                display_logs: _,
                tx,
                expect,
            } => Step::ScCall {
                tx_id,
                comment,
                tx: Box::new(TxCall::interpret_from(tx, context)),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            },
            StepRaw::ScQuery {
                tx_id,
                comment,
                display_logs: _,
                tx,
                expect,
            } => Step::ScQuery {
                tx_id,
                comment,
                tx: Box::new(TxQuery::interpret_from(tx, context)),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            },
            StepRaw::ScDeploy {
                tx_id,
                comment,
                display_logs: _,
                tx,
                expect,
            } => Step::ScDeploy {
                tx_id,
                comment,
                tx: Box::new(TxDeploy::interpret_from(tx, context)),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            },
            StepRaw::Transfer { tx_id, comment, tx } => Step::Transfer {
                tx_id,
                comment,
                tx: Box::new(TxTransfer::interpret_from(tx, context)),
            },
            StepRaw::ValidatorReward { tx_id, comment, tx } => Step::ValidatorReward {
                tx_id,
                comment,
                tx: Box::new(TxValidatorReward::interpret_from(tx, context)),
            },
            StepRaw::CheckState { comment, accounts } => Step::CheckState {
                comment,
                accounts: CheckAccounts::interpret_from(accounts, context),
            },
            StepRaw::DumpState { comment } => Step::DumpState { comment },
        }
    }
}
