use crate::*;
use super::sc_error::SCError;
use super::finish::EndpointResult;

/// Default way to optionally return an error from a smart contract endpoint.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum SCResult<T> {
    Ok(T),
    Err(SCError),
}

impl<T> SCResult<T> {
    #[inline]
    pub fn is_ok(&self) -> bool {
        if let SCResult::Ok(_) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub fn ok(self) -> Option<T> {
        if let SCResult::Ok(t) = self {
            Some(t)
        } else {
            None
        }
    }

    #[inline]
    pub fn err(self) -> Option<SCError> {
        if let SCResult::Err(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

impl<'a, A, BigInt, BigUint, T> EndpointResult<'a, A, BigInt, BigUint> for SCResult<T>
where
    T: EndpointResult<'a, A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    #[inline]
    fn finish(&self, api: &'a A) {
        match self {
            SCResult::Ok(t) => {
                t.finish(api);
            },
            SCResult::Err(e) => {
                api.signal_error(e.as_bytes());
            }
        }
    }
}

impl<T> SCResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            SCResult::Ok(t) => t,
            SCResult::Err(_) => panic!("called `SCResult::unwrap()`"),
        }
    }
}
