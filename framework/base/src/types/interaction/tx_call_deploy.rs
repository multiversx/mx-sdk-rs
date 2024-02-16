use multiversx_sc_codec::{CodecFrom, TopEncodeMulti};

use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    tuple_util::NestedTupleFlatten,
    types::{CodeMetadata, ManagedAddress, ManagedBuffer, ManagedVec},
};

use super::{
    contract_call_exec::decode_result, Code, ConsNoRet, ConsRet, DeployCall, FromSource,
    OriginalResultMarker, RHList, RHListItem, Tx, TxDataFunctionCall, TxEmptyResultHandler, TxEnv,
    TxGas, TxPayment, TxPaymentEgldOnly, TxResultHandler, TxScEnv, TxToSpecified,
};

pub trait RHListItemDeploy<Env, Original>: RHListItem<Env, Original>
where
    Env: TxEnv,
{
    fn item_deploy_result(
        self,
        new_address: &ManagedAddress<Env::Api>,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns;
}

pub trait RHListDeploy<Env>: RHList<Env>
where
    Env: TxEnv,
{
    fn list_deploy_result(
        self,
        new_address: &ManagedAddress<Env::Api>,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns;
}

impl<Env> RHListDeploy<Env> for ()
where
    Env: TxEnv,
{
    fn list_deploy_result(
        self,
        _new_address: &ManagedAddress<Env::Api>,
        _raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
    }
}

impl<Env, O> RHListDeploy<Env> for OriginalResultMarker<O>
where
    Env: TxEnv,
{
    fn list_deploy_result(
        self,
        _new_address: &ManagedAddress<Env::Api>,
        _raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
    }
}

impl<Env, Head, Tail> RHListDeploy<Env> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemDeploy<Env, Tail::OriginalResult>,
    Tail: RHListDeploy<Env>,
{
    fn list_deploy_result(
        self,
        new_address: &ManagedAddress<Env::Api>,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
        let head_result = self.head.item_deploy_result(new_address, raw_results);
        let tail_result = self.tail.list_deploy_result(new_address, raw_results);
        (head_result, tail_result)
    }
}

impl<Env, Head, Tail> RHListDeploy<Env> for ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemDeploy<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHListDeploy<Env>,
{
    fn list_deploy_result(
        self,
        new_address: &ManagedAddress<Env::Api>,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
        self.head.item_deploy_result(new_address, raw_results);
        self.tail.list_deploy_result(new_address, raw_results)
    }
}

impl<Api, Payment, Gas, RH>
    Tx<TxScEnv<Api>, (), (), Payment, Gas, DeployCall<TxScEnv<Api>, Code<TxScEnv<Api>>>, RH>
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: TxResultHandler<TxScEnv<Api>>,
{
    fn execute_deploy_raw(self) -> (ManagedAddress<Api>, ManagedVec<Api, ManagedBuffer<Api>>, RH) {
        let gas_limit = self.gas.resolve_gas(&self.env);
        let egld_payment = self.payment.to_egld_payment();

        let (new_address, raw_results) = SendRawWrapper::<Api>::new().deploy_contract(
            gas_limit,
            &egld_payment.value,
            &self.data.code_source.code,
            self.data.code_metadata,
            &self.data.arg_buffer,
        );

        SendRawWrapper::<Api>::new().clean_return_data();

        (new_address, raw_results, self.result_handler)
    }
}

impl<Api, Payment, Gas, RH>
    Tx<TxScEnv<Api>, (), (), Payment, Gas, DeployCall<TxScEnv<Api>, FromSource<TxScEnv<Api>>>, RH>
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: TxResultHandler<TxScEnv<Api>>,
{
    fn execute_deploy_from_source_raw(
        self,
    ) -> (ManagedAddress<Api>, ManagedVec<Api, ManagedBuffer<Api>>, RH) {
        let gas_limit = self.gas.resolve_gas(&self.env);
        let egld_payment = self.payment.to_egld_payment();

        let (new_address, raw_results) = SendRawWrapper::<Api>::new().deploy_from_source_contract(
            gas_limit,
            &egld_payment.value,
            &self.data.code_source.address,
            self.data.code_metadata,
            &self.data.arg_buffer,
        );

        SendRawWrapper::<Api>::new().clean_return_data();

        (new_address, raw_results, self.result_handler)
    }
}

impl<Api, Payment, Gas, RH>
    Tx<TxScEnv<Api>, (), (), Payment, Gas, DeployCall<TxScEnv<Api>, Code<TxScEnv<Api>>>, RH>
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: RHListDeploy<TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    /// Synchronously deploys a contract.
    pub fn sync_call(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (new_address, raw_results, result_handler) = self.execute_deploy_raw();

        let tuple_result = result_handler.list_deploy_result(&new_address, &raw_results);
        tuple_result.flatten_unpack()
    }
}

impl<Api, Payment, Gas, RH>
    Tx<TxScEnv<Api>, (), (), Payment, Gas, DeployCall<TxScEnv<Api>, FromSource<TxScEnv<Api>>>, RH>
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: RHListDeploy<TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    /// Synchronously deploys a contract from source.
    pub fn sync_call(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (new_address, raw_results, result_handler) = self.execute_deploy_from_source_raw();

        let tuple_result = result_handler.list_deploy_result(&new_address, &raw_results);
        tuple_result.flatten_unpack()
    }
}

impl<Api, Payment, Gas, RH>
    Tx<
        TxScEnv<Api>,
        (),
        ManagedAddress<Api>,
        Payment,
        Gas,
        DeployCall<TxScEnv<Api>, Code<TxScEnv<Api>>>,
        RH,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    pub fn upgrade_async_call(self) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        SendRawWrapper::<Api>::new().upgrade_contract(
            &self.to,
            gas,
            &self.payment.to_egld_payment().value,
            &self.data.code_source.code,
            self.data.code_metadata,
            &self.data.arg_buffer,
        );
    }
}

impl<Api, Payment, Gas, RH>
    Tx<
        TxScEnv<Api>,
        (),
        ManagedAddress<Api>,
        Payment,
        Gas,
        DeployCall<TxScEnv<Api>, FromSource<TxScEnv<Api>>>,
        RH,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    pub fn upgrade_async_call(self) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        SendRawWrapper::<Api>::new().upgrade_from_source_contract(
            &self.to,
            gas,
            &self.payment.to_egld_payment().value,
            &self.data.code_source.address,
            self.data.code_metadata,
            &self.data.arg_buffer,
        );
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
    /// This is because the old API was passing it as paramter, so we do the Apime here.
    /// For clarity, we don't want it set twice.
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
    /// This is because the old API was passing it as paramter, so we do the Apime here.
    /// For clarity, we don't want it set twice.
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

impl<Api, Payment, Gas, RH>
    Tx<TxScEnv<Api>, (), ManagedAddress<Api>, Payment, Gas, DeployCall<TxScEnv<Api>, ()>, RH>
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Backwards compatibility, immitates the old API.
    ///
    /// Note that the data type (the `DeployCall`) doesn't have the code set.
    /// This is because the old API was passing it as paramter, so we do the Apime here.
    /// For clarity, we don't want it set twice.
    pub fn upgrade_contract(self, code: &ManagedBuffer<Api>, code_metadata: CodeMetadata) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        SendRawWrapper::<Api>::new().upgrade_contract(
            &self.to,
            gas,
            &self.payment.to_egld_payment().value,
            code,
            code_metadata,
            &self.data.arg_buffer,
        );
    }

    /// Backwards compatibility, immitates the old API.
    ///
    /// Note that the data type (the `DeployCall`) doesn't have the code set.
    /// This is because the old API was passing it as paramter, so we do the Apime here.
    /// For clarity, we don't want it set twice.
    pub fn upgrade_from_source(
        self,
        source_address: &ManagedAddress<Api>,
        code_metadata: CodeMetadata,
    ) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        SendRawWrapper::<Api>::new().upgrade_from_source_contract(
            &self.to,
            gas,
            &self.payment.to_egld_payment().value,
            source_address,
            code_metadata,
            &self.data.arg_buffer,
        );
    }
}
