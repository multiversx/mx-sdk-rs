use core::marker::PhantomData;

use crate::types::{OriginalResultMarker, TxEnv, TxResultHandler};

use super::RHListItem;

pub trait RHList<Env>: TxResultHandler<Env>
where
    Env: TxEnv,
{
    type ListReturns;
}

pub trait RHListAppendRet<Env, T>: RHList<Env>
where
    Env: TxEnv,
    T: RHListItem<Env, Self::OriginalResult>,
{
    type RetOutput: RHList<Env, OriginalResult = Self::OriginalResult>;

    fn append_ret(self, t: T) -> Self::RetOutput;
}

pub trait RHListAppendNoRet<Env, T>: RHList<Env>
where
    Env: TxEnv,
    T: RHListItem<Env, Self::OriginalResult, Returns = ()>,
{
    type NoRetOutput: RHList<Env, OriginalResult = Self::OriginalResult>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput;
}

impl<Env> RHList<Env> for ()
where
    Env: TxEnv,
{
    type ListReturns = ();
}

impl<Env, T> RHListAppendRet<Env, T> for ()
where
    Env: TxEnv,
    T: RHListItem<Env, ()>,
{
    type RetOutput = ConsRet<Env, T, ()>;

    fn append_ret(self, t: T) -> Self::RetOutput {
        ConsRet::new(t, self)
    }
}

impl<Env, T> RHListAppendNoRet<Env, T> for ()
where
    Env: TxEnv,
    T: RHListItem<Env, (), Returns = ()>,
{
    type NoRetOutput = ConsNoRet<Env, T, ()>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput {
        ConsNoRet::new(t, self)
    }
}

impl<Env, O> RHList<Env> for OriginalResultMarker<O>
where
    Env: TxEnv,
{
    type ListReturns = ();
}

impl<Env, O, T> RHListAppendRet<Env, T> for OriginalResultMarker<O>
where
    Env: TxEnv,
    T: RHListItem<Env, O>,
{
    type RetOutput = ConsRet<Env, T, OriginalResultMarker<O>>;

    fn append_ret(self, t: T) -> Self::RetOutput {
        ConsRet::new(t, self)
    }
}

impl<Env, O, T> RHListAppendNoRet<Env, T> for OriginalResultMarker<O>
where
    Env: TxEnv,
    T: RHListItem<Env, O, Returns = ()>,
{
    type NoRetOutput = ConsNoRet<Env, T, OriginalResultMarker<O>>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput {
        ConsNoRet::new(t, self)
    }
}

pub struct ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult>,
    Tail: RHList<Env>,
{
    _phantom: PhantomData<Env>,
    pub head: Head,
    pub tail: Tail,
}

impl<Env, Head, Tail> ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult>,
    Tail: RHList<Env>,
{
    fn new(head: Head, tail: Tail) -> Self {
        ConsRet {
            _phantom: PhantomData,
            head,
            tail,
        }
    }
}

impl<Env, Head, Tail> TxResultHandler<Env> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult>,
    Tail: RHList<Env>,
{
    type OriginalResult = Tail::OriginalResult;
}

impl<Env, Head, Tail> RHList<Env> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult>,
    Tail: RHList<Env>,
{
    type ListReturns = (Head::Returns, Tail::ListReturns);
}

impl<Env, Head, Tail, T> RHListAppendRet<Env, T> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult>,
    Tail: RHList<Env> + RHListAppendRet<Env, T>,
    T: RHListItem<Env, Tail::OriginalResult>,
{
    type RetOutput = ConsRet<Env, Head, <Tail as RHListAppendRet<Env, T>>::RetOutput>;

    fn append_ret(self, t: T) -> Self::RetOutput {
        ConsRet::new(self.head, self.tail.append_ret(t))
    }
}

impl<Env, Head, Tail, T> RHListAppendNoRet<Env, T> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult>,
    Tail: RHList<Env> + RHListAppendNoRet<Env, T>,
    T: RHListItem<Env, Tail::OriginalResult, Returns = ()>,
{
    type NoRetOutput = ConsRet<Env, Head, <Tail as RHListAppendNoRet<Env, T>>::NoRetOutput>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput {
        ConsRet::new(self.head, self.tail.append_no_ret(t))
    }
}

/// Handlers that return nothing.
pub struct ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHList<Env>,
{
    _phantom: PhantomData<Env>,
    pub head: Head,
    pub tail: Tail,
}

impl<Env, Head, Tail> ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHList<Env>,
{
    fn new(head: Head, tail: Tail) -> Self {
        ConsNoRet {
            _phantom: PhantomData,
            head,
            tail,
        }
    }
}

impl<Env, Head, Tail> TxResultHandler<Env> for ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHList<Env>,
{
    type OriginalResult = Tail::OriginalResult;
}

impl<Env, Head, Tail> RHList<Env> for ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHList<Env>,
{
    type ListReturns = Tail::ListReturns;
}

impl<Env, Head, Tail, T> RHListAppendRet<Env, T> for ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHList<Env> + RHListAppendRet<Env, T>,
    T: RHListItem<Env, Tail::OriginalResult>,
{
    type RetOutput = ConsNoRet<Env, Head, <Tail as RHListAppendRet<Env, T>>::RetOutput>;

    fn append_ret(self, t: T) -> Self::RetOutput {
        ConsNoRet::new(self.head, self.tail.append_ret(t))
    }
}

impl<Env, Head, Tail, T> RHListAppendNoRet<Env, T> for ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItem<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHList<Env> + RHListAppendNoRet<Env, T>,
    T: RHListItem<Env, Tail::OriginalResult, Returns = ()>,
{
    type NoRetOutput = ConsNoRet<Env, Head, <Tail as RHListAppendNoRet<Env, T>>::NoRetOutput>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput {
        ConsNoRet::new(self.head, self.tail.append_no_ret(t))
    }
}
