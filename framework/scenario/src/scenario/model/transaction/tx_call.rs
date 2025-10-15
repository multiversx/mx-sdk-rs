use multiversx_chain_vm::types::{top_encode_big_uint, top_encode_u64};
use multiversx_sc::api::{
    ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME, ESDT_TRANSFER_FUNC_NAME,
};

use crate::{
    scenario::model::{AddressValue, BigUintValue, BytesValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxCallRaw,
    },
};

use super::{tx_interpret_util::interpret_egld_value, TxESDT};

pub const DEFAULT_GAS_EXPR: &str = "5,000,000";

#[derive(Debug, Clone)]
pub struct TxCall {
    pub from: AddressValue,
    pub to: AddressValue,
    pub egld_value: BigUintValue,
    pub esdt_value: Vec<TxESDT>,
    pub function: String,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl Default for TxCall {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            egld_value: Default::default(),
            esdt_value: Default::default(),
            function: Default::default(),
            arguments: Default::default(),
            gas_limit: U64Value::from(DEFAULT_GAS_EXPR),
            gas_price: Default::default(),
        }
    }
}

impl InterpretableFrom<TxCallRaw> for TxCall {
    fn interpret_from(from: TxCallRaw, context: &InterpreterContext) -> Self {
        TxCall {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            egld_value: interpret_egld_value(from.value, from.egld_value, context),
            esdt_value: from
                .esdt_value
                .into_iter()
                .map(|esdt_value| TxESDT::interpret_from(esdt_value, context))
                .collect(),
            function: from.function,
            arguments: from
                .arguments
                .into_iter()
                .map(|t| BytesValue::interpret_from(t, context))
                .collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price.unwrap_or_default(), context),
        }
    }
}

impl IntoRaw<TxCallRaw> for TxCall {
    fn into_raw(self) -> TxCallRaw {
        TxCallRaw {
            from: self.from.into_raw(),
            to: self.to.into_raw(),
            value: None,
            egld_value: self.egld_value.into_raw_opt(),
            esdt_value: self
                .esdt_value
                .into_iter()
                .map(|esdt_value| esdt_value.into_raw())
                .collect(),
            function: self.function,
            arguments: self
                .arguments
                .into_iter()
                .map(|arg| arg.into_raw())
                .collect(),
            gas_limit: self.gas_limit.into_raw(),
            gas_price: self.gas_price.into_raw_opt(),
        }
    }
}

impl TxCall {
    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
    #[allow(deprecated)]
    #[cfg(feature = "contract-call-legacy")]
    pub fn to_contract_call(
        &self,
    ) -> multiversx_sc::types::ContractCallWithEgld<crate::imports::StaticApi, ()> {
        use multiversx_sc::types::ContractCall;

        let mut contract_call = multiversx_sc::types::ContractCallWithEgld::new(
            (&self.to.value).into(),
            self.function.as_bytes(),
            (&self.egld_value.value).into(),
        );

        contract_call.basic.explicit_gas_limit = self.gas_limit.value;

        contract_call = contract_call.convert_to_esdt_transfer_call(
            self.esdt_value
                .iter()
                .map(|esdt| {
                    crate::imports::EsdtTokenPayment::new(
                        esdt.esdt_token_identifier.value.as_slice().into(),
                        esdt.nonce.value,
                        (&esdt.esdt_value.value).into(),
                    )
                })
                .collect(),
        );

        // For some contract calls from == to.
        // The contract call objects have no "from" field, since that is always part of the execution context.
        // On the static API there is no execution context, but a placeholder value is provided.
        // Here we already know the sender, so we can replace the placeholder with the actual value.
        if crate::imports::StaticApi::is_current_address_placeholder(
            &contract_call.basic.to.to_address(),
        ) {
            contract_call.basic.to = self.from.value.clone().into();
        }

        for argument in &self.arguments {
            contract_call.push_raw_argument(argument.value.as_slice());
        }
        contract_call
    }

    /// Converts call to builtin function ESDT transfer call, if necessary.
    pub fn normalize(&self) -> TxCall {
        let (function, arguments, to_self) = self.process_payments();
        TxCall {
            from: self.from.clone(),
            to: if to_self {
                self.from.clone()
            } else {
                self.to.clone()
            },
            egld_value: self.egld_value.clone(),
            esdt_value: Vec::new(),
            function,
            arguments,
            gas_limit: self.gas_limit.clone(),
            gas_price: self.gas_price.clone(),
        }
    }

    fn process_payments(&self) -> (String, Vec<BytesValue>, bool) {
        assert!(
            self.egld_value.is_zero() || self.esdt_value.is_empty(),
            "Cannot have both EGLD and ESDT fields filled. To transfer EGLD and ESDT in the same transaction, represent EGLD as EGLD-000000 in the ESDTs.");

        match self.esdt_value.len() {
            0 => (self.function.clone(), self.arguments.clone(), false),
            1 => {
                let payment = self.esdt_value.first().unwrap();
                if payment.is_egld() {
                    self.construct_multi_transfer_esdt_call()
                } else if payment.nonce.value == 0 {
                    self.construct_single_transfer_fungible_call(payment)
                } else {
                    self.construct_single_transfer_nft_call(payment)
                }
            }
            _ => self.construct_multi_transfer_esdt_call(),
        }
    }

    fn append_function_call_to_arguments(&self, arguments: &mut Vec<BytesValue>) {
        if !self.function.is_empty() {
            arguments.push(BytesValue::from_str_expr(&self.function));
        }
        for regular_arg in &self.arguments {
            arguments.push(regular_arg.clone())
        }
    }

    /// Constructs `ESDTTransfer` builtin function call.
    pub(crate) fn construct_single_transfer_fungible_call(
        &self,
        payment: &TxESDT,
    ) -> (String, Vec<BytesValue>, bool) {
        let mut arguments = Vec::new();
        arguments.push(payment.esdt_token_identifier.value.as_slice().into());
        arguments.push(top_encode_big_uint(&payment.esdt_value.value).into());
        self.append_function_call_to_arguments(&mut arguments);

        (ESDT_TRANSFER_FUNC_NAME.to_owned(), arguments, false)
    }

    /// Constructs `ESDTNFTTransfer` builtin function call.
    ///
    /// `ESDTNFTTransfer` takes 4 arguments:
    /// arg0 - token identifier
    /// arg1 - nonce
    /// arg2 - quantity to transfer
    /// arg3 - destination address
    pub(crate) fn construct_single_transfer_nft_call(
        &self,
        payment: &TxESDT,
    ) -> (String, Vec<BytesValue>, bool) {
        let mut arguments = vec![
            payment.esdt_token_identifier.value.as_slice().into(),
            top_encode_u64(payment.nonce.value).into(),
            top_encode_big_uint(&payment.esdt_value.value).into(),
            self.to.value.as_bytes().into(), // TODO: preserve representation
        ];

        self.append_function_call_to_arguments(&mut arguments);

        (ESDT_NFT_TRANSFER_FUNC_NAME.to_owned(), arguments, true)
    }

    /// Constructs `MultiESDTNFTTransfer` builtin function call.
    pub(crate) fn construct_multi_transfer_esdt_call(&self) -> (String, Vec<BytesValue>, bool) {
        let mut arguments = Vec::new();
        arguments.push(self.to.value.as_bytes().into()); // TODO: preserve representation
        arguments.push(top_encode_u64(self.esdt_value.len() as u64).into());

        for payment in &self.esdt_value {
            arguments.push(payment.esdt_token_identifier.value.as_slice().into());
            arguments.push(top_encode_u64(payment.nonce.value).into());
            arguments.push(top_encode_big_uint(&payment.esdt_value.value).into());
        }

        self.append_function_call_to_arguments(&mut arguments);

        (ESDT_MULTI_TRANSFER_FUNC_NAME.to_owned(), arguments, true)
    }

    /// Creates the data field of the transaction represented by object.
    pub fn compute_data_field(&self) -> String {
        let mut result = self.function.clone();
        for argument in &self.arguments {
            result.push('@');
            result.push_str(hex::encode(argument.value.as_slice()).as_str());
        }
        result
    }
}
