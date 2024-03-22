use crate::types::{
    BigUint, EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment, EsdtTokenPayment, ManagedAddress,
    MultiEsdtPayment,
};

use super::{
    Egld, FunctionCall, TxEgldValue, TxEnv, TxFrom, TxPayment,  TxToSpecified,
};

/// Defines how a payment transforms a transaction,
/// e.g. from ESDT transfer to ESDTTransfer builtin function.
pub trait TxPaymentNormalize<Env, From, To>: TxPayment<Env>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R;
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for ()
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        _from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| f(to_addr, &BigUint::zero(), &fc))
    }
}

impl<Env, From, To, EgldValue> TxPaymentNormalize<Env, From, To> for Egld<EgldValue>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
    EgldValue: TxEgldValue<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        _from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            self.0
                .with_egld_value(|egld_value| f(to_addr, egld_value, &fc))
        })
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for EsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            if self.token_nonce == 0 {
                let fc_conv = fc.convert_to_single_transfer_fungible_call(self);
                f(to_addr, &BigUint::zero(), &fc_conv)
            } else {
                let fc_conv = fc.convert_to_single_transfer_nft_call(to_addr, self);
                f(&from.resolve_address(env), &BigUint::zero(), &fc_conv)
            }
        })
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for MultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        match self.len() {
            0 => ().with_normalized(env, from, to, fc, f),
            1 => self.get(0).with_normalized(env, from, to, fc, f),
            _ => to.with_address_ref(env, |to_addr| {
                let fc_conv = fc.convert_to_multi_transfer_esdt_call(to_addr, self);
                f(&from.resolve_address(env), &BigUint::zero(), &fc_conv)
            }),
        }
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for EgldOrEsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        self.map_egld_or_esdt(
            (to, fc, f),
            |(to, fc, f), amount| Egld(amount).with_normalized(env, from, to, fc, f),
            |(to, fc, f), esdt_payment| esdt_payment.with_normalized(env, from, to, fc, f),
        )
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for EgldOrMultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                Egld(egld_amount).with_normalized(env, from, to, fc, f)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.with_normalized(env, from, to, fc, f)
            },
        }
    }
}
