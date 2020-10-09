use super::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Scenario {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub check_gas: Option<bool>,
    pub steps: Vec<Step>,
}

impl InterpretableFrom<ScenarioRaw> for Scenario {
    fn interpret_from(from: ScenarioRaw, context: &InterpreterContext) -> Self {
        Scenario {
            name: from.name,
            comment: from.comment,
            check_gas: from.check_gas,
            steps: from.steps.into_iter().map(|s| Step::interpret_from(s, context)).collect(),
        }
    }
}

#[derive(Debug)]
pub enum Step {
    ExternalSteps {
        path: String,
    },
    SetState {
        comment: Option<String>,
        owner: Option<AddressValue>,
        accounts: BTreeMap<AddressKey, Account>,
        new_addresses: Vec<NewAddress>,
        block_hashes: Vec<BytesValue>,
        previous_block_info: Option<BlockInfo>,
        current_block_info: Option<BlockInfo>,
    },
    ScCall {
        tx_id: String,
        comment: Option<String>,
        tx: TxCall,
        expect: Option<TxExpect>,
    },
    ScDeploy {
        tx_id: String,
        comment: Option<String>,
        tx: TxDeploy,
        expect: Option<TxExpect>,
    },
    Transfer {
        tx_id: String,
        comment: Option<String>,
        tx: TxTransfer,
    },
    ValidatorReward {
        tx_id: String,
        comment: Option<String>,
        tx: TxValidatorReward,
    },
    CheckState {
        comment: Option<String>,
        owner: Option<AddressValue>,
        accounts: CheckAccounts,
    },
    DumpState {
        comment: Option<String>,
    },
}

impl InterpretableFrom<StepRaw> for Step {
    fn interpret_from(from: StepRaw, context: &InterpreterContext) -> Self {
        match from {
            StepRaw::ExternalSteps {
                path,
            } => Step::ExternalSteps {
                path
            },
            StepRaw::SetState {
                comment,
                owner,
                accounts,
                new_addresses,
                block_hashes,
                previous_block_info,
                current_block_info,
            } => Step::SetState {
                comment,
                owner: owner.map(|o| AddressValue::interpret_from(o, context)),
                accounts: accounts.into_iter().map(|(k, v)| (
                    AddressKey::interpret_from(k, context),
                    Account::interpret_from(v, context))).collect(),
                new_addresses: new_addresses.into_iter().map(|t| NewAddress::interpret_from(t, context)).collect(),
                block_hashes: block_hashes.into_iter().map(|t| BytesValue::interpret_from(t, context)).collect(),
                previous_block_info: previous_block_info.map(|v| BlockInfo::interpret_from(v, context)),
                current_block_info: current_block_info.map(|v| BlockInfo::interpret_from(v, context)),
            },
            StepRaw::ScCall {
                tx_id,
                comment,
                tx,
                expect,
            } => Step::ScCall {
                tx_id,
                comment,
                tx: TxCall::interpret_from(tx, context),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            },
            StepRaw::ScDeploy {
                tx_id,
                comment,
                tx,
                expect,
            } => Step::ScDeploy {
                tx_id,
                comment,
                tx: TxDeploy::interpret_from(tx, context),
                expect: expect.map(|v| TxExpect::interpret_from(v, context)),
            },
            StepRaw::Transfer {
                tx_id,
                comment,
                tx,
            } => Step::Transfer {
                tx_id,
                comment,
                tx: TxTransfer::interpret_from(tx, context),
            },
            StepRaw::ValidatorReward {
                tx_id,
                comment,
                tx,
            } => Step::ValidatorReward {
                tx_id,
                comment,
                tx: TxValidatorReward::interpret_from(tx, context),
            },
            StepRaw::CheckState {
                comment,
                owner,
                accounts,
            } => Step::CheckState {
                comment,
                owner: owner.map(|o| AddressValue::interpret_from(o, context)),
                accounts: CheckAccounts::interpret_from(accounts, context),
            },
            StepRaw::DumpState {
                comment,
            } => Step::DumpState {
                comment,
            },
        }
    }
}

#[derive(Debug)]
pub struct NewAddress {
    pub creator_address: AddressValue,
    pub creator_nonce: U64Value,
    pub new_address: AddressValue,
}

impl InterpretableFrom<NewAddressRaw> for NewAddress {
    fn interpret_from(from: NewAddressRaw, context: &InterpreterContext) -> Self {
        NewAddress {
            creator_address: AddressValue::interpret_from(from.creator_address, context),
            creator_nonce: U64Value::interpret_from(from.creator_nonce, context),
            new_address: AddressValue::interpret_from(from.new_address, context),
        }
    }
}

#[derive(Debug)]
pub struct BlockInfo {
    pub block_timestamp: Option<U64Value>,
    pub block_nonce: Option<U64Value>,
    pub block_round: Option<U64Value>,
    pub block_epoch: Option<U64Value>,
}

impl InterpretableFrom<BlockInfoRaw> for BlockInfo {
    fn interpret_from(from: BlockInfoRaw, context: &InterpreterContext) -> Self {
        BlockInfo {
            block_timestamp: from.block_timestamp.map(|v| U64Value::interpret_from(v, context)),
            block_nonce: from.block_nonce.map(|v| U64Value::interpret_from(v, context)),
            block_round: from.block_round.map(|v| U64Value::interpret_from(v, context)),
            block_epoch: from.block_epoch.map(|v| U64Value::interpret_from(v, context)),
        }
    }
}

#[derive(Debug)]
pub struct TxCall {
    pub from: AddressValue,
    pub to: AddressValue,
    pub call_value: BigUintValue,
    pub function: String,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxCallRaw> for TxCall {
    fn interpret_from(from: TxCallRaw, context: &InterpreterContext) -> Self {
        TxCall {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            call_value: BigUintValue::interpret_from(from.value, context),
            function: from.function,
            arguments: from.arguments.into_iter().map(|t| BytesValue::interpret_from(t, context)).collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price, context),
        }
    }
}

#[derive(Debug)]
pub struct TxDeploy {
    pub from: AddressValue,
    pub call_value: BigUintValue,
    pub contract_code: BytesValue,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxDeployRaw> for TxDeploy {
    fn interpret_from(from: TxDeployRaw, context: &InterpreterContext) -> Self {
        TxDeploy {
            from: AddressValue::interpret_from(from.from, context),
            call_value: BigUintValue::interpret_from(from.value, context),
            contract_code: BytesValue::interpret_from(from.contract_code, context),
            arguments: from.arguments.into_iter().map(|t| BytesValue::interpret_from(t, context)).collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price, context),
        }
    }
}

#[derive(Debug)]
pub struct TxTransfer {
    pub from: AddressValue,
    pub to: AddressValue,
    pub value: BigUintValue,
}

impl InterpretableFrom<TxTransferRaw> for TxTransfer {
    fn interpret_from(from: TxTransferRaw, context: &InterpreterContext) -> Self {
        TxTransfer {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            value: BigUintValue::interpret_from(from.value, context),
        }
    }
}

#[derive(Debug)]
pub struct TxValidatorReward {
    pub to: AddressValue,
    pub value: BigUintValue,
}

impl InterpretableFrom<TxValidatorRewardRaw> for TxValidatorReward {
    fn interpret_from(from: TxValidatorRewardRaw, context: &InterpreterContext) -> Self {
        TxValidatorReward {
            to: AddressValue::interpret_from(from.to, context),
            value: BigUintValue::interpret_from(from.value, context),
        }
    }
}

#[derive(Debug)]
pub struct TxExpect {
    pub out: Vec<CheckValue<BytesValue>>,
    pub status: U64Value,
    pub logs: CheckLogs,
    pub message: Option<BytesValue>,
    pub gas: Option<CheckValue<U64Value>>,
    pub refund: Option<CheckValue<U64Value>>,

}

impl InterpretableFrom<TxExpectRaw> for TxExpect {
    fn interpret_from(from: TxExpectRaw, context: &InterpreterContext) -> Self {
        TxExpect {
            out: from.out.into_iter().map(|t| CheckValue::<BytesValue>::interpret_from(t, context)).collect(),
            status: U64Value::interpret_from(from.status, context),
            logs: CheckLogs::interpret_from(from.logs, context),
            message: from.message.map(|v| BytesValue::interpret_from(v, context)),
            gas: from.gas.map(|v| CheckValue::<U64Value>::interpret_from(v, context)),
            refund: from.refund.map(|v| CheckValue::<U64Value>::interpret_from(v, context)),
        }
    }
}
