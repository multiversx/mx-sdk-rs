use multiversx_sc::{abi::TypeAbiFrom, types::H256};
use unwrap_infallible::UnwrapInfallible;

use crate::{
    api::StaticApi,
    scenario::model::{AddressValue, BigUintValue, BytesValue, TxCall, TxESDT, TxExpect, U64Value},
    scenario_model::TxResponse,
};

use crate::multiversx_sc::{
    codec::{PanicErrorHandler, TopEncodeMulti},
    types::{ContractCall, ManagedArgBuffer},
};

#[derive(Debug, Clone)]
pub struct ScCallStep {
    pub id: String,
    pub tx_id: Option<String>,
    pub explicit_tx_hash: Option<H256>,
    pub comment: Option<String>,
    pub tx: Box<TxCall>,
    pub expect: Option<TxExpect>,
    pub response: Option<TxResponse>,
}

impl Default for ScCallStep {
    fn default() -> Self {
        Self {
            id: Default::default(),
            tx_id: Default::default(),
            explicit_tx_hash: Default::default(),
            comment: Default::default(),
            tx: Default::default(),
            expect: Some(TxExpect::ok()),
            response: Default::default(),
        }
    }
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
        if !self.tx.esdt_value.is_empty() && self.tx.egld_value.value > 0u32.into() {
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

    pub fn multi_esdt_transfer<T>(mut self, tokens: T) -> Self
    where
        T: IntoIterator<Item = TxESDT>,
    {
        if self.tx.egld_value.value > 0u32.into() {
            panic!("Cannot transfer both EGLD and ESDT");
        }

        self.tx.esdt_value.extend(tokens);

        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.tx.function = expr.to_string();
        self
    }

    pub fn tx_hash<T>(mut self, tx_hash_expr: T) -> Self
    where
        H256: From<T>,
    {
        self.explicit_tx_hash = Some(tx_hash_expr.into());
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

    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
    #[allow(deprecated)]
    pub fn call<CC>(mut self, contract_call: CC) -> super::TypedScCall<CC::OriginalResult>
    where
        CC: multiversx_sc::types::ContractCallBase<StaticApi>,
    {
        let (to_str, function, egld_value_expr, scenario_args) =
            process_contract_call(contract_call);
        self = self.to(to_str.as_str());

        if self.tx.function.is_empty() {
            self = self.function(function.as_str());
        }
        if self.tx.egld_value.value == 0u32.into() {
            self = self.egld_value(egld_value_expr);
        }
        for arg in scenario_args {
            self = self.argument(arg.as_str());
        }
        self.into()
    }

    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    /// - "expect"
    ///     - "out"
    ///     - "status" set to 0
    #[deprecated(
        since = "0.42.0",
        note = "Please use `call` followed by `expect`, there is no point in having a method that does both."
    )]
    #[allow(deprecated)]
    pub fn call_expect<CC, ExpectedResult>(
        self,
        contract_call: CC,
        expected_value: ExpectedResult,
    ) -> super::TypedScCall<CC::OriginalResult>
    where
        CC: ContractCall<StaticApi>,
        ExpectedResult: TypeAbiFrom<CC::OriginalResult> + TopEncodeMulti,
    {
        self.call(contract_call).expect_value(expected_value)
    }

    /// Adds a custom expect section to the tx.
    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.expect = Some(expect);
        self
    }

    /// Explicitly states that no tx expect section should be added and no checks should be performed.
    ///
    /// Note: by default a basic `TxExpect::ok()` is added, which checks that status is 0 and nothing else.
    pub fn no_expect(mut self) -> Self {
        self.expect = None;
        self
    }

    /// Unwraps the response, if available.
    pub fn response(&self) -> &TxResponse {
        self.response
            .as_ref()
            .expect("SC call response not yet available")
    }

    pub fn save_response(&mut self, tx_response: TxResponse) {
        if let Some(expect) = &mut self.expect {
            if expect.build_from_response {
                expect.update_from_response(&tx_response)
            }
        }
        self.response = Some(tx_response);
    }
}

impl AsMut<ScCallStep> for ScCallStep {
    fn as_mut(&mut self) -> &mut ScCallStep {
        self
    }
}

/// Extracts
/// - recipient,
/// - endpoint name,
/// - the arguments.
#[allow(deprecated)]
pub(super) fn process_contract_call<CC>(
    contract_call: CC,
) -> (String, String, BigUintValue, Vec<String>)
where
    CC: multiversx_sc::types::ContractCallBase<StaticApi>,
{
    let normalized_cc = contract_call.into_normalized();
    let to_str = format!(
        "0x{}",
        hex::encode(normalized_cc.basic.to.to_address().as_bytes())
    );
    let function = String::from_utf8(
        normalized_cc
            .basic
            .function_call
            .function_name
            .to_boxed_bytes()
            .into_vec(),
    )
    .unwrap();
    let egld_value_expr = BigUintValue::from(normalized_cc.egld_payment);
    let scenario_args = convert_call_args(&normalized_cc.basic.function_call.arg_buffer);
    (to_str, function, egld_value_expr, scenario_args)
}

pub fn convert_call_args(arg_buffer: &ManagedArgBuffer<StaticApi>) -> Vec<String> {
    arg_buffer
        .to_raw_args_vec()
        .iter()
        .map(|arg| format!("0x{}", hex::encode(arg)))
        .collect()
}

pub(super) fn format_expect<T: TopEncodeMulti>(t: T) -> TxExpect {
    let mut encoded = Vec::<Vec<u8>>::new();
    t.multi_encode_or_handle_err(&mut encoded, PanicErrorHandler)
        .unwrap_infallible();
    let mut expect = TxExpect::ok().no_result();
    for encoded_res in encoded {
        let encoded_hex_string = format!("0x{}", hex::encode(encoded_res.as_slice()));
        expect = expect.result(encoded_hex_string.as_str());
    }
    expect
}
