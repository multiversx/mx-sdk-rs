use crate::{
    mandos_system::model::{ScCallStep, ScDeployStep, ScQueryStep, TxExpect},
    DebugApi,
};
use elrond_wasm::{
    elrond_codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    types::{ContractCall, ContractDeploy, ManagedArgBuffer},
};

impl ScCallStep {
    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    pub fn call<OriginalResult>(
        mut self,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> Self {
        let (to_str, function, mandos_args) = process_contract_call(contract_call);
        self = self.to(to_str.as_str());
        self = self.function(function.as_str());
        for arg in mandos_args {
            self = self.argument(arg.as_str());
        }
        self
    }
}

impl ScCallStep {
    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    /// - "expect"
    ///     - "out"
    ///     - "status" set to 0
    pub fn call_expect<OriginalResult, ExpectedResult>(
        mut self,
        contract_call: ContractCall<DebugApi, OriginalResult>,
        expect_value: ExpectedResult,
    ) -> Self
    where
        OriginalResult: TopEncodeMulti,
        ExpectedResult: CodecFrom<OriginalResult> + TopEncodeMulti,
    {
        self = self.call(contract_call);
        self = self.expect(format_expect(expect_value));
        self
    }
}

impl ScQueryStep {
    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    pub fn call<OriginalResult>(
        mut self,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> Self {
        let (to_str, function, mandos_args) = process_contract_call(contract_call);
        self = self.to(to_str.as_str());
        self = self.function(function.as_str());
        for arg in mandos_args {
            self = self.argument(arg.as_str());
        }
        self
    }
}

impl ScQueryStep {
    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    /// - "expect"
    ///     - "out"
    ///     - "status" set to 0
    pub fn call_expect<OriginalResult, ExpectedResult>(
        mut self,
        contract_call: ContractCall<DebugApi, OriginalResult>,
        expect_value: ExpectedResult,
    ) -> Self
    where
        OriginalResult: TopEncodeMulti,
        ExpectedResult: CodecFrom<OriginalResult> + TopEncodeMulti,
    {
        self = self.call(contract_call);
        self = self.expect(format_expect(expect_value));
        self
    }
}

impl ScDeployStep {
    /// Sets following fields based on the smart contract proxy:
    /// - "function"
    /// - "arguments"
    pub fn call<OriginalResult>(
        mut self,
        contract_deploy: ContractDeploy<DebugApi, OriginalResult>,
    ) -> Self {
        let (_, mandos_args) = process_contract_deploy(contract_deploy);
        for arg in mandos_args {
            self = self.argument(arg.as_str());
        }
        self
    }
}

pub fn convert_call_args(arg_buffer: &ManagedArgBuffer<DebugApi>) -> Vec<String> {
    arg_buffer
        .to_raw_args_vec()
        .iter()
        .map(|arg| format!("0x{}", hex::encode(&arg)))
        .collect()
}

/// Extracts
/// - recipient,
/// - endpoint name,
/// - the arguments.
fn process_contract_call<OriginalResult>(
    contract_call: ContractCall<DebugApi, OriginalResult>,
) -> (String, String, Vec<String>) {
    let to_str = format!(
        "0x{}",
        hex::encode(contract_call.to.to_address().as_bytes())
    );
    let function =
        String::from_utf8(contract_call.endpoint_name.to_boxed_bytes().into_vec()).unwrap();
    let mandos_args = convert_call_args(&contract_call.arg_buffer);
    (to_str, function, mandos_args)
}

/// Extracts
/// - (optional) recipient (needed for contract upgrade, not yet used);
/// - the arguments.
fn process_contract_deploy<OriginalResult>(
    contract_deploy: ContractDeploy<DebugApi, OriginalResult>,
) -> (Option<String>, Vec<String>) {
    let to_str = contract_deploy
        .to
        .as_option()
        .map(|to| format!("0x{}", hex::encode(to.to_address().as_bytes())));
    let mandos_args = convert_call_args(&contract_deploy.arg_buffer);
    (to_str, mandos_args)
}

fn format_expect<T: TopEncodeMulti>(t: T) -> TxExpect {
    let mut encoded = Vec::<Vec<u8>>::new();
    let Ok(()) = t.multi_encode_or_handle_err(&mut encoded, PanicErrorHandler);
    let mut expect = TxExpect::ok().no_result();
    for encoded_res in encoded {
        let encoded_hex_string = format!("0x{}", hex::encode(encoded_res.as_slice()));
        expect = expect.result(encoded_hex_string.as_str());
    }
    expect
}
