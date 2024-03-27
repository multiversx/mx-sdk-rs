use multiversx_sc_codec::{CodecFrom, TopEncodeMulti};

use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    tuple_util::NestedTupleFlatten,
    types::{CodeMetadata, ManagedAddress, ManagedBuffer, ManagedVec},
};

use super::{
    contract_call_exec::decode_result, Code, ConsNoRet, ConsRet, DeployCall, FromSource,
    OriginalResultMarker, RHList, RHListExec, RHListItem, Tx, TxCodeValue, TxEmptyResultHandler,
    TxEnv, TxFromSourceValue, TxGas, TxPaymentEgldOnly, TxResultHandler, TxScEnv, UpgradeCall,
};

pub struct DeployRawResult<Api>
where
    Api: CallTypeApi,
{
    pub new_address: ManagedAddress<Api>,
    pub raw_results: ManagedVec<Api, ManagedBuffer<Api>>,
}

impl<Api, Payment, Gas, CodeValue, RH>
    Tx<TxScEnv<Api>, (), (), Payment, Gas, DeployCall<TxScEnv<Api>, Code<CodeValue>>, RH>
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    CodeValue: TxCodeValue<TxScEnv<Api>>,
    RH: TxResultHandler<TxScEnv<Api>>,
{
    fn execute_deploy_raw(self) -> (ManagedAddress<Api>, ManagedVec<Api, ManagedBuffer<Api>>, RH) {
        let gas_limit = self.gas.gas_value(&self.env);

        let (new_address, raw_results) = self.payment.with_egld_value(&self.env, |egld_value| {
            SendRawWrapper::<Api>::new().deploy_contract(
                gas_limit,
                egld_value,
                &self.data.code_source.0.into_value(&self.env),
                self.data.code_metadata,
                &self.data.arg_buffer,
            )
        });

        SendRawWrapper::<Api>::new().clean_return_data();

        (new_address, raw_results, self.result_handler)
    }
}

impl<Api, Payment, Gas, FromSourceValue, RH>
    Tx<
        TxScEnv<Api>,
        (),
        (),
        Payment,
        Gas,
        DeployCall<TxScEnv<Api>, FromSource<FromSourceValue>>,
        RH,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FromSourceValue: TxFromSourceValue<TxScEnv<Api>>,
    RH: TxResultHandler<TxScEnv<Api>>,
{
    fn execute_deploy_from_source_raw(
        self,
    ) -> (ManagedAddress<Api>, ManagedVec<Api, ManagedBuffer<Api>>, RH) {
        let gas_limit = self.gas.gas_value(&self.env);

        let (new_address, raw_results) = self.payment.with_egld_value(&self.env, |egld_value| {
            SendRawWrapper::<Api>::new().deploy_from_source_contract(
                gas_limit,
                egld_value,
                &self.data.code_source.0.into_value(&self.env),
                self.data.code_metadata,
                &self.data.arg_buffer,
            )
        });

        SendRawWrapper::<Api>::new().clean_return_data();

        (new_address, raw_results, self.result_handler)
    }
}

impl<Api, Payment, Gas, CodeValue, RH>
    Tx<TxScEnv<Api>, (), (), Payment, Gas, DeployCall<TxScEnv<Api>, Code<CodeValue>>, RH>
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    CodeValue: TxCodeValue<TxScEnv<Api>>,
    RH: RHListExec<DeployRawResult<Api>, TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    /// Synchronously deploys a contract.
    pub fn sync_call(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (new_address, raw_results, result_handler) = self.execute_deploy_raw();

        let deploy_raw_result = DeployRawResult {
            new_address,
            raw_results,
        };
        let tuple_result = result_handler.list_process_result(&deploy_raw_result);
        tuple_result.flatten_unpack()
    }
}

impl<Api, Payment, Gas, FromSourceValue, RH>
    Tx<
        TxScEnv<Api>,
        (),
        (),
        Payment,
        Gas,
        DeployCall<TxScEnv<Api>, FromSource<FromSourceValue>>,
        RH,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FromSourceValue: TxFromSourceValue<TxScEnv<Api>>,
    RH: RHListExec<DeployRawResult<Api>, TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    /// Synchronously deploys a contract from source.
    pub fn sync_call(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (new_address, raw_results, result_handler) = self.execute_deploy_from_source_raw();

        let deploy_raw_result = DeployRawResult {
            new_address,
            raw_results,
        };
        let tuple_result = result_handler.list_process_result(&deploy_raw_result);
        tuple_result.flatten_unpack()
    }
}

impl<Api, Payment, Gas, OriginalResult>
    Tx<
        TxScEnv<Api>,
        (),
        (),
        Payment,
        Gas,
        DeployCall<TxScEnv<Api>, ()>,
        OriginalResultMarker<OriginalResult>,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    OriginalResult: TopEncodeMulti,
{
    /// Backwards compatibility, immitates the old API.
    ///
    /// Note that the data type (the `DeployCall`) doesn't have the code set.
    /// This is because the old API was passing it as paramter, so we use it from the `code` argument.
    ///
    /// Also note that the code metadata is taken from the `code_metadata` argument.
    /// If another one was previously set in the `Tx` object, that one will be ignored.
    pub fn deploy_contract<RequestedResult>(
        self,
        code: &ManagedBuffer<Api>,
        code_metadata: CodeMetadata,
    ) -> (ManagedAddress<Api>, RequestedResult)
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let (new_address, raw_results, _) = self
            .code(code.clone())
            .code_metadata(code_metadata)
            .execute_deploy_raw();

        (new_address, decode_result(raw_results))
    }

    /// Backwards compatibility, immitates the old API.
    ///
    /// Note that the data type (the `DeployCall`) doesn't have the code set.
    /// This is because the old API was passing it as paramter, so we use it from the `code` argument.
    ///
    /// Also note that the code metadata is taken from the `code_metadata` argument.
    /// If another one was previously set in the `Tx` object, that one will be ignored.
    pub fn deploy_from_source<RequestedResult>(
        self,
        source_address: &ManagedAddress<Api>,
        code_metadata: CodeMetadata,
    ) -> (ManagedAddress<Api>, RequestedResult)
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let (new_address, raw_results, _) = self
            .from_source(source_address.clone())
            .code_metadata(code_metadata)
            .execute_deploy_from_source_raw();

        (new_address, decode_result(raw_results))
    }
}
