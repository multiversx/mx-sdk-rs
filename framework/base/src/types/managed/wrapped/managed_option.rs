use core::marker::PhantomData;

use crate::{
    abi::TypeAbiFrom,
    api::HandleConstraints,
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
        NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    },
};

use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{const_handles, use_raw_handle, ErrorApiImpl, ManagedTypeApi},
    types::{ManagedRef, ManagedType},
};

use super::{ManagedVecItem, ManagedVecItemPayloadBuffer};

/// A very efficient optional managed type.
///
/// `None` is flagged by a special invalid handle.
pub struct ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub(super) _phantom_m: PhantomData<M>,
    pub(super) handle: T::OwnHandle,
}

impl<M, T> ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn new_with_handle(handle: T::OwnHandle) -> Self {
        Self {
            _phantom_m: PhantomData,
            handle,
        }
    }

    pub fn some(value: T) -> Self {
        unsafe { Self::new_with_handle(value.forget_into_handle()) }
    }

    pub fn none() -> Self {
        Self::new_with_handle(use_raw_handle(const_handles::MANAGED_OPTION_NONE))
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
        self.handle.clone() == const_handles::MANAGED_OPTION_NONE
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Assumes that value is Some and unwraps without checking.
    ///
    /// # Safety
    ///
    /// Must always be called under an `if` checking `.is_some()`, otherwise will lead to undefined behaviour.
    pub unsafe fn unwrap_no_check(self) -> T {
        T::from_handle(self.handle)
    }

    pub fn into_option(self) -> Option<T> {
        if self.is_some() {
            Some(unsafe { self.unwrap_no_check() })
        } else {
            None
        }
    }

    pub fn as_option(&self) -> Option<ManagedRef<'_, M, T>> {
        if self.is_some() {
            unsafe { Some(ManagedRef::wrap_handle(self.handle.clone())) }
        } else {
            None
        }
    }

    pub fn unwrap_or_else<F: Fn() -> T>(self, f: F) -> T {
        if self.is_some() {
            unsafe { self.unwrap_no_check() }
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
            ManagedOption::<M, U>::some(f(unsafe { self.unwrap_no_check() }))
        } else {
            ManagedOption::<M, U>::none()
        }
    }

    pub fn map_or_else<Context, D, F, R>(self, context: Context, default: D, f: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, T) -> R,
    {
        if self.is_some() {
            f(context, unsafe { self.unwrap_no_check() })
        } else {
            default(context)
        }
    }

    pub fn map_ref_or_else<Context, D, F, R>(&self, context: Context, default: D, f: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, &T) -> R,
    {
        if self.is_some() {
            unsafe {
                let obj = T::from_handle(self.handle.clone());
                let result = f(context, &obj);
                let _ = obj.forget_into_handle();
                result
            }
        } else {
            default(context)
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
        self.map_ref_or_else((), |()| Self::none(), |(), obj| Self::some(obj.clone()))
    }
}

impl<M, T> PartialEq for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.handle.clone() == other.handle.clone() {
            // also catches None == None
            return true;
        }
        if self.is_some() && other.is_some() {
            unsafe {
                return ManagedRef::<'_, _, T>::wrap_handle(self.handle.clone())
                    == ManagedRef::<'_, _, T>::wrap_handle(other.handle.clone());
            }
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

impl<M, T> ManagedVecItem for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + 'static,
{
    type PAYLOAD = ManagedVecItemPayloadBuffer<4>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let handle = use_raw_handle(i32::read_from_payload(payload));
        Self::new_with_handle(handle)
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        // TODO: managed ref
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.handle.get_raw_handle().save_to_payload(payload);
    }
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

impl<M, T, U> TypeAbiFrom<ManagedOption<M, U>> for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    U: ManagedType<M>,
    T: ManagedType<M> + TypeAbiFrom<U>,
{
}

impl<M, T, U> TypeAbiFrom<Option<U>> for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TypeAbiFrom<U>,
{
}

impl<M, T, U> TypeAbiFrom<ManagedOption<M, U>> for Option<T>
where
    M: ManagedTypeApi,
    U: ManagedType<M>,
    T: TypeAbiFrom<U>,
{
}

impl<M, T> TypeAbi for ManagedOption<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TypeAbi,
{
    type Unmanaged = Option<T::Unmanaged>;

    /// It is semantically equivalent to any list of `T`.
    fn type_name() -> TypeName {
        Option::<T>::type_name()
    }

    fn type_name_rust() -> TypeName {
        TypeName::from(alloc::format!(
            "ManagedOption<$API, {}>",
            T::type_name_rust()
        ))
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
        self.map_ref_or_else(
            f,
            |f| f.write_str("ManagedOption::None"),
            |f, obj| f.debug_tuple("ManagedOption::Some").field(obj).finish(),
        )
    }
}
