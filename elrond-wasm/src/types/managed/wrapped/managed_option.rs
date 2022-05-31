use core::marker::PhantomData;

use elrond_codec::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{const_handles, ErrorApiImpl, Handle, ManagedTypeApi},
    types::{ManagedRef, ManagedType},
};

/// A very efficient optional managed type.
///
/// `None` is flagged by a special invalid handle.
pub struct ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub(super) _phantom_m: PhantomData<M>,
    pub(super) _phantom_t: PhantomData<T>,
    pub(super) handle: Handle,
}

impl<M, T> ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub fn some(value: T) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: value.get_raw_handle(),
        }
    }

    pub fn none() -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: const_handles::MANAGED_OPTION_NONE,
        }
    }
}

impl<M, T> From<Option<T>> for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn from(opt: Option<T>) -> Self {
        if let Some(value) = opt {
            Self::some(value)
        } else {
            Self::none()
        }
    }
}

impl<M, T> ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub fn is_none(&self) -> bool {
        self.handle == const_handles::MANAGED_OPTION_NONE
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn into_option(self) -> Option<T> {
        if self.is_some() {
            Some(T::from_raw_handle(self.handle))
        } else {
            None
        }
    }

    pub fn as_option(&self) -> Option<ManagedRef<'_, M, T>> {
        if self.is_some() {
            unsafe { Some(ManagedRef::wrap_handle(self.handle)) }
        } else {
            None
        }
    }

    pub fn unwrap_or_else<F: Fn() -> T>(self, f: F) -> T {
        if self.is_some() {
            T::from_raw_handle(self.handle)
        } else {
            f()
        }
    }

    pub fn unwrap_or_sc_panic(self, panic_message: &str) -> T {
        self.unwrap_or_else(|| M::error_api_impl().signal_error(panic_message.as_bytes()))
    }

    pub fn map<U, F>(self, f: F) -> ManagedOption<M, U>
    where
        U: ManagedType<M>,
        F: FnOnce(T) -> U,
    {
        if self.is_some() {
            ManagedOption::<M, U>::some(f(T::from_raw_handle(self.handle)))
        } else {
            ManagedOption::<M, U>::none()
        }
    }

    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        if self.is_some() {
            f(T::from_raw_handle(self.handle))
        } else {
            default()
        }
    }

    pub fn map_ref_or_else<U, D, F>(&self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&T) -> U,
    {
        if self.is_some() {
            f(&T::from_raw_handle(self.handle))
        } else {
            default()
        }
    }
}

impl<M, T> Clone for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + Clone,
{
    #[allow(clippy::redundant_clone)] // the clone is not redundant
    fn clone(&self) -> Self {
        if self.is_some() {
            Self::some(T::from_raw_handle(self.handle).clone())
        } else {
            Self::none()
        }
    }
}

impl<M, T> PartialEq for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.handle == other.handle {
            // also catches None == None
            return true;
        }
        if self.is_some() && other.is_some() {
            return T::from_raw_handle(self.handle) == T::from_raw_handle(other.handle);
        }
        false
    }
}

impl<M, T> Eq for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
}

impl<M, T> TopEncode for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedEncode,
{
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_option().top_encode_or_handle_err(output, h)
    }
}

impl<M, T> NestedEncode for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedEncode,
{
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_option().dep_encode_or_handle_err(dest, h)
    }
}

impl<M, T> TopDecode for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedDecode,
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::from(Option::<T>::top_decode_or_handle_err(input, h)?))
    }
}

impl<M, T> NestedDecode for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedDecode,
{
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::from(Option::<T>::dep_decode_or_handle_err(input, h)?))
    }
}

impl<M, T> TypeAbi for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TypeAbi,
{
    /// It is semantically equivalent to any list of `T`.
    fn type_name() -> TypeName {
        Option::<T>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<M, T> core::fmt::Debug for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_some() {
            f.debug_tuple("ManagedOption::Some")
                .field(&T::from_raw_handle(self.handle))
                .finish()
        } else {
            f.write_str("ManagedOption::None")
        }
    }
}
