/// A tuple of the form (A, (B, (... (N, ())))).
///
/// It is always terminated with a unit.
pub trait NestedTuple {}

impl NestedTuple for () {}

impl<Head, Tail> NestedTuple for (Head, Tail) where Tail: NestedTuple {}

/// Allows to append at the end of a nested tuple list.
pub trait NestedTupleAppend<T> {
    type Output;

    fn append(self, t: T) -> Self::Output;
}

impl<T> NestedTupleAppend<T> for () {
    type Output = (T, ());

    fn append(self, t: T) -> Self::Output {
        (t, ())
    }
}

impl<Head, Tail, T> NestedTupleAppend<T> for (Head, Tail)
where
    Tail: NestedTupleAppend<T>,
{
    type Output = (Head, Tail::Output);

    fn append(self, t: T) -> Self::Output {
        (self.0, self.1.append(t))
    }
}

/// Defines conversion of a nested tuple list to a regular tuple.
pub trait NestedTupleFlatten: NestedTuple {
    type Flattened;
    type Unpacked;

    /// Converts a nested tuple list to a regular tuple.
    fn flatten(self) -> Self::Flattened;

    /// Same as `flatten`, converts a nested tuple list to a regular tuple,
    /// but additionally, it unpacks singleton tuples into their content (`(item,)` -> `item`).
    fn flatten_unpack(self) -> Self::Unpacked;
}

impl NestedTupleFlatten for () {
    type Flattened = ();
    type Unpacked = ();

    fn flatten(self) -> Self::Flattened {}
    fn flatten_unpack(self) -> Self::Unpacked {}
}

impl<T> NestedTupleFlatten for (T, ()) {
    type Flattened = (T,);
    type Unpacked = T;

    fn flatten(self) -> Self::Flattened {
        (self.0,)
    }

    fn flatten_unpack(self) -> Self::Unpacked {
        self.0
    }
}

macro_rules! tuple_list_type {
    () => ( () );
    ($i:ty)  => ( ($i, ()) );
    ($i:ty, $($e:ty),*)  => ( ($i, tuple_list_type!($($e),*)) );
}

macro_rules! unnest {
    (($layer:expr); ($($v:expr),*); ($u:ident, $($us:ident,)*)) => {
        unnest!(($layer . 1); ($($v,)* $layer . 0); ($($us,)*))
    };
    (($layer:expr); ($($v:expr),*); ()) => { ($($v,)*) };
}

macro_rules! flatten_impl {
    ($(($t:ident $($ts:ident)+))+) => {
        $(
            impl<$t,$($ts),+> NestedTupleFlatten for tuple_list_type!($t,$($ts),+) {
                type Flattened = ($t,$($ts),+);
                type Unpacked = ($t,$($ts),+);

                fn flatten(self) -> Self::Flattened {
                    unnest!((self); (); ($t, $($ts,)*))
                }

                fn flatten_unpack(self) -> Self::Unpacked {
                    self.flatten()
                }
            }
        )+
    }
}

flatten_impl! {
    (T1 T2)
    (T1 T2 T3)
    (T1 T2 T3 T4)
    (T1 T2 T3 T4 T5)
    (T1 T2 T3 T4 T5 T6)
    (T1 T2 T3 T4 T5 T6 T7)
    (T1 T2 T3 T4 T5 T6 T7 T8)
    (T1 T2 T3 T4 T5 T6 T7 T8 T9)
    (T1 T2 T3 T4 T5 T6 T7 T8 T9 T10)
    (T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11)
    (T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12)
    (T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13)
    (T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14)
    (T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15)
    (T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flatten() {
        let flat2 = (1, (2, ())).flatten();
        assert_eq!(flat2, (1, 2));

        let n3 = (1u8, (2u16, (3u32, ())));
        let flat3 = n3.flatten();
        assert_eq!(flat3, (1u8, 2u16, 3u32));

        let n4 = n3.append(4u64);
        let flat4 = n4.flatten();
        assert_eq!(flat4, (1u8, 2u16, 3u32, 4u64));
    }
}
