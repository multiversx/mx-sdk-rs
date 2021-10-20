use core::{borrow::Borrow, marker::PhantomData, ops::Deref};

pub struct ManagedRef<T, B>
where
    B: Borrow<T>,
{
    _phantom: PhantomData<T>,
    borrow: B,
}

impl<T, B> Deref for ManagedRef<T, B>
where
    B: Borrow<T>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow.borrow()
    }
}

impl<T, B> From<B> for ManagedRef<T, B>
where
    B: Borrow<T>,
{
    fn from(borrow: B) -> Self {
        ManagedRef {
            _phantom: PhantomData,
            borrow,
        }
    }
}
