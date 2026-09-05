#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::abi::*;

pub struct AdderAbiProxy;

impl<T> AbiProxyTrait<T> for AdderAbiProxy {
    type Methods = AdderAbiProxyMethods<T>;

    fn proxy_methods(self, base_tx: T) -> Self::Methods {
        AdderAbiProxyMethods { base_tx }
    }
}

pub struct AdderAbiProxyMethods<T> {
    base_tx: T,
}

impl<T> AdderAbiProxyMethods<T>
where
    T: IntoDeploy<NotPayable, ()>,
{
    pub fn init<Arg0: ProxyArg<BigUintAbi>>(self, initial_value: Arg0) -> T::Out {
        self.base_tx
            .into_deploy(NotPayable)
            .apply_argument(&initial_value)
    }
}

impl<T> AdderAbiProxyMethods<T>
where
    T: IntoUpgrade<NotPayable, ()>,
{
    pub fn upgrade<Arg0: ProxyArg<BigUintAbi>>(self, initial_value: Arg0) -> T::Out {
        self.base_tx
            .into_upgrade(NotPayable)
            .apply_argument(&initial_value)
    }
}

impl<T> AdderAbiProxyMethods<T>
where
    T: IntoCall<NotPayable, BigUintAbi>,
{
    pub fn sum(self) -> <T as IntoCall<NotPayable, BigUintAbi>>::Out {
        self.base_tx.into_call(NotPayable, "getSum")
    }
}

impl<T> AdderAbiProxyMethods<T>
where
    T: IntoCall<NotPayable, ()>,
{
    pub fn add<Arg0: ProxyArg<BigUintAbi>>(self, value: Arg0) -> T::Out {
        self.base_tx
            .into_call(NotPayable, "add")
            .apply_argument(&value)
    }
}
