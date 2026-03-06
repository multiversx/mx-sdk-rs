mod async_call;
mod async_call_promises;
mod contract_call_convert;
mod contract_call_exec;
mod contract_call_no_payment;
mod contract_call_trait;
mod contract_call_with_any_payment;
mod contract_call_with_egld;
mod contract_call_with_egld_or_single_esdt;
mod contract_call_with_multi_esdt;
mod contract_deploy;

pub use async_call::AsyncCall;
pub use async_call_promises::AsyncCallPromises;
pub use contract_call_no_payment::ContractCallNoPayment;
pub use contract_call_trait::{ContractCall, ContractCallBase};
pub use contract_call_with_any_payment::ContractCallWithAnyPayment;
pub use contract_call_with_egld::ContractCallWithEgld;
pub use contract_call_with_egld_or_single_esdt::ContractCallWithEgldOrSingleEsdt;
pub use contract_call_with_multi_esdt::ContractCallWithMultiEsdt;
pub use contract_deploy::{ContractDeploy, new_contract_deploy};
use multiversx_sc_codec::TopEncodeMulti;

/// Using max u64 to represent maximum possible gas,
/// so that the value zero is not reserved and can be specified explicitly.
/// Leaving the gas limit unspecified will replace it with `api.get_gas_left()`.
pub(crate) const UNSPECIFIED_GAS_LIMIT: u64 = u64::MAX;

use crate::{
    api::CallTypeApi,
    types::{
        DeployCall, FunctionCall, ManagedOption, OriginalResultMarker, Tx, TxPayment,
        TxPaymentEgldOnly, TxScEnv, TxTo, TxToSpecified,
    },
};

// Conversion from new syntax to old syntax.
impl<Api, To, Payment, OriginalResult> ContractCallBase<Api>
    for Tx<
        TxScEnv<Api>,
        (),
        To,
        Payment,
        (),
        FunctionCall<Api>,
        OriginalResultMarker<OriginalResult>,
    >
where
    Api: CallTypeApi + 'static,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithEgld<Api, OriginalResult> {
        self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data,
            |norm_to, norm_egld, norm_fc| ContractCallWithEgld {
                basic: ContractCallNoPayment {
                    _phantom: core::marker::PhantomData,
                    to: norm_to.clone(),
                    function_call: norm_fc.clone(),
                    explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
                    _return_type: core::marker::PhantomData,
                },
                egld_payment: norm_egld.clone(),
            },
        )
    }
}

impl<Api, To, Payment, OriginalResult>
    From<
        Tx<
            TxScEnv<Api>,
            (),
            To,
            Payment,
            (),
            DeployCall<TxScEnv<Api>, ()>,
            OriginalResultMarker<OriginalResult>,
        >,
    > for ContractDeploy<Api, OriginalResult>
where
    Api: CallTypeApi + 'static,
    To: TxTo<TxScEnv<Api>>,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    OriginalResult: TopEncodeMulti,
{
    fn from(
        value: Tx<
            TxScEnv<Api>,
            (),
            To,
            Payment,
            (),
            DeployCall<TxScEnv<Api>, ()>,
            OriginalResultMarker<OriginalResult>,
        >,
    ) -> Self {
        ContractDeploy {
            _phantom: core::marker::PhantomData,
            to: ManagedOption::none(),
            egld_payment: value.payment.into_egld_payment(&value.env),
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            arg_buffer: value.data.arg_buffer,
            _return_type: core::marker::PhantomData,
        }
    }
}
