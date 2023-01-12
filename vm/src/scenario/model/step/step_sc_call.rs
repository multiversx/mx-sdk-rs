use crate::{
    scenario::model::{AddressValue, BigUintValue, BytesValue, TxCall, TxESDT, TxExpect, U64Value},
    DebugApi,
};

use multiversx_sc::{
    codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    types::{ContractCall, ManagedArgBuffer},
};

#[derive(Debug, Default)]
pub struct ScCallStep {
    pub id: String,
    pub tx_id: Option<String>,
    pub comment: Option<String>,
    pub tx: Box<TxCall>,
    pub expect: Option<TxExpect>,
}

impl ScCallStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.tx.from = AddressValue::from(address);
        self
    }

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.tx.to = AddressValue::from(address);
        self
    }

    pub fn egld_value<A>(mut self, amount: A) -> Self
    where
        BigUintValue: From<A>,
    {
        if !self.tx.esdt_value.is_empty() {
            panic!("Cannot transfer both EGLD and ESDT");
        }

        self.tx.egld_value = BigUintValue::from(amount);
        self
    }

    pub fn esdt_transfer<T, N, A>(mut self, token_id: T, token_nonce: N, amount: A) -> Self
    where
        BytesValue: From<T>,
        U64Value: From<N>,
        BigUintValue: From<A>,
    {
        if self.tx.egld_value.value > 0u32.into() {
            panic!("Cannot transfer both EGLD and ESDT");
        }

        self.tx.esdt_value.push(TxESDT {
            esdt_token_identifier: BytesValue::from(token_id),
            nonce: U64Value::from(token_nonce),
            esdt_value: BigUintValue::from(amount),
        });

        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.tx.function = expr.to_string();
        self
    }

    pub fn argument<A>(mut self, expr: A) -> Self
    where
        BytesValue: From<A>,
    {
        self.tx.arguments.push(BytesValue::from(expr));
        self
    }

    pub fn gas_limit<V>(mut self, value: V) -> Self
    where
        U64Value: From<V>,
    {
        self.tx.gas_limit = U64Value::from(value);
        self
    }

    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.expect = Some(expect);
        self
    }

    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    pub fn call<CC>(mut self, contract_call: CC) -> Self
    where
        CC: ContractCall<DebugApi>,
    {
        let (to_str, function, scenario_args) = process_contract_call(contract_call);
        self = self.to(to_str.as_str());
        self = self.function(function.as_str());
        for arg in scenario_args {
            self = self.argument(arg.as_str());
        }
        self
    }

    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    /// - "expect"
    ///     - "out"
    ///     - "status" set to 0
    pub fn call_expect<CC, ExpectedResult>(
        mut self,
        contract_call: CC,
        expect_value: ExpectedResult,
    ) -> Self
    where
        CC: ContractCall<DebugApi>,
        ExpectedResult: CodecFrom<CC::OriginalResult> + TopEncodeMulti,
    {
        self = self.call(contract_call);
        self = self.expect(format_expect(expect_value));
        self
    }
}

/// Extracts
/// - recipient,
/// - endpoint name,
/// - the arguments.
pub(super) fn process_contract_call<CC>(contract_call: CC) -> (String, String, Vec<String>)
where
    CC: ContractCall<DebugApi>,
{
    let full_cc = contract_call.into_normalized();
    let to_str = format!(
        "0x{}",
        hex::encode(full_cc.basic.to.to_address().as_bytes())
    );
    let function =
        String::from_utf8(full_cc.basic.endpoint_name.to_boxed_bytes().into_vec()).unwrap();
    let scenario_args = convert_call_args(&full_cc.basic.arg_buffer);
    (to_str, function, scenario_args)
}

pub fn convert_call_args(arg_buffer: &ManagedArgBuffer<DebugApi>) -> Vec<String> {
    arg_buffer
        .to_raw_args_vec()
        .iter()
        .map(|arg| format!("0x{}", hex::encode(arg)))
        .collect()
}

pub(super) fn format_expect<T: TopEncodeMulti>(t: T) -> TxExpect {
    let mut encoded = Vec::<Vec<u8>>::new();
    let Ok(()) = t.multi_encode_or_handle_err(&mut encoded, PanicErrorHandler);
    let mut expect = TxExpect::ok().no_result();
    for encoded_res in encoded {
        let encoded_hex_string = format!("0x{}", hex::encode(encoded_res.as_slice()));
        expect = expect.result(encoded_hex_string.as_str());
    }
    expect
}
