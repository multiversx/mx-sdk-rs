use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    tuple_util::NestedTupleFlatten,
    types::{ManagedAddress, ManagedBuffer, ManagedVec},
};

use super::{
    ConsNoRet, ConsRet, DeployCall, OriginalResultMarker, RHList, RHListItem, Tx,
    TxDataFunctionCall, TxEnv, TxGas, TxPayment, TxPaymentEgldOnly, TxScEnv, TxToSpecified,
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

impl<Api, Payment, Gas, RH> Tx<TxScEnv<Api>, (), (), Payment, Gas, DeployCall<TxScEnv<Api>>, RH>
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: RHListDeploy<TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub fn execute_deploy(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let gas_limit = self.gas.resolve_gas(&self.env);
        let egld_payment = self.payment.to_egld_payment();

        let (new_address, raw_results) = SendRawWrapper::<Api>::new().deploy_contract(
            gas_limit,
            &egld_payment.value,
            &self.data.code,
            self.data.code_metadata,
            &self.data.arg_buffer,
        );

        SendRawWrapper::<Api>::new().clean_return_data();

        let tuple_result = self
            .result_handler
            .list_deploy_result(&new_address, &raw_results);
        tuple_result.flatten_unpack()
    }
}
