use crate::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

/// Pure ABI marker for counted variadic arguments and results, i.e. `counted-variadic<T>` in the ABI.
///
/// Zero-sized counterpart of `MultiValueEncodedCounted`, the same way `VariadicAbi` is for `MultiValueEncoded`.
///
/// ## Layout
///
/// Unlike `variadic<T>`, which simply consumes all remaining multi-value arguments,
/// `counted-variadic<T>` encodes its length explicitly, so it can be followed by other arguments:
///
/// ```text
/// count | item_1 | item_2 | ... | item_count
/// ```
///
/// - `count` is a single top-encoded `usize` argument: the number of `T` items that follow.
/// - Each item is `T` multi-encoded, i.e. it occupies as many arguments as `T`'s multi-value length:
///   a simple type (`u32`, `BigUint`, ...) takes 1, a `MultiValue3<A, B, C>` takes 3.
///   `T` must therefore have a constant multi-value length (a nested `variadic` is not allowed).
///
/// The total number of raw arguments consumed is `1 + count * multi_length(T)`.
///
/// Example: `counted-variadic<multi<u32,bytes>>` holding `(1, "a")` and `(2, "b")` is passed as 5 arguments:
///
/// ```text
/// 2 | 1 | "a" | 2 | "b"
/// ```
pub struct CountedVariadicAbi<T>
where
    T: TypeAbi,
{
    _phantom: core::marker::PhantomData<T>,
}

impl<T, U> TypeAbiFrom<CountedVariadicAbi<U>> for CountedVariadicAbi<T>
where
    T: TypeAbi + TypeAbiFrom<U>,
    U: TypeAbi,
{
}

impl<T> TypeAbi for CountedVariadicAbi<T>
where
    T: TypeAbi,
{
    type Abi = Self;

    fn type_name() -> TypeName {
        let mut repr = TypeName::from("counted-variadic<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn type_name_rust() -> TypeName {
        let mut repr = TypeName::from("CountedVariadicAbi<");
        repr.push_str(T::type_name_rust().as_str());
        repr.push('>');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
