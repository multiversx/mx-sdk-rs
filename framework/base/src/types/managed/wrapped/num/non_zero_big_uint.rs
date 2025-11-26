use multiversx_sc_codec::DecodeError;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{quick_signal_error, ManagedTypeApi},
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
        NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    },
    err_msg,
    formatter::{hex_util::encode_bytes_as_hex, FormatByteReceiver, SCDisplay},
    types::{BigInt, BigUint, ManagedBuffer, ManagedType, NonZeroError},
};

/// A big, unsigned number that is guaranteed not to be zero.
///
/// The restriction is enforced by the constructors and the implementation of operations.
///
/// ## Constructor
///
/// To build a NonZeroBigUint, the best way is via NonZeroBigUint::new(BigUint), which returns an Option:
/// - Some(NonZeroBigUint) if the input is non-zero, or
/// - None if the input is zero.
///
/// This way the user can handle the zero case as they see fit.
///
/// The quicker alternative is NonZeroBigUint::new_or_panic(BigUint), which will signal an error if the input is zero.
///
/// ## Binary Operators
///
/// Naturally, some operations between two NonZeroBigUint can never yield zero,
/// so no validation is needed after those operations.
///
/// For all others, there is an additional check at the end, which will crash execution if the invariant is violated.
///
/// Specifically, for the binary operators, we have implemented:
///
/// - NonZeroBigUint + NonZeroBigUint = guaranteed non-zero, no validation needed
/// - NonZeroBigUint * NonZeroBigUint = guaranteed non-zero, no validation needed
/// - NonZeroBigUint / NonZeroBigUint = could yield zero, validation needed
/// - NonZeroBigUint * NonZeroBigUint = could yield zero or negative, validation needed
///
/// ## Assign Operators
///
/// For the assign operators, we have similar logic, but we've added a few more operations for convenience:
/// - BigUint
/// - u32
/// - u64
///
/// Again, no validation is needed when the operation guarantees a non-zero result:
/// - NonZeroBigUint += NonZeroBigUint
/// - NonZeroBigUint += &NonZeroBigUint
/// - NonZeroBigUint += BigUint
/// - NonZeroBigUint += u32
/// - NonZeroBigUint += u64
/// - NonZeroBigUint *= NonZeroBigUint
/// - NonZeroBigUint *= &NonZeroBigUint
///
/// Everything else gets a runtime check.
///
/// ## Usage in Payments
///
/// Since token payments cannot be zero, NonZeroBigUint is used in the Payment object.
///
/// Getting a call value is guaranteed to yield non-zero amounts,
/// so in that case NonZeroBigUint is getting its invariant from the VM.
#[repr(transparent)]
pub struct NonZeroBigUint<M: ManagedTypeApi> {
    pub(super) value: BigInt<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for NonZeroBigUint<M> {
    type OwnHandle = M::BigIntHandle;

    unsafe fn from_handle(handle: M::BigIntHandle) -> Self {
        NonZeroBigUint {
            value: BigInt::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::BigIntHandle {
        self.value.handle.clone()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        self.value.forget_into_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigIntHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::BigIntHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> TryFrom<BigUint<M>> for NonZeroBigUint<M> {
    type Error = NonZeroError;

    fn try_from(bu: BigUint<M>) -> Result<Self, Self::Error> {
        Self::new(bu).map_or_else(|| Err(NonZeroError), Ok)
    }
}

impl<M: ManagedTypeApi> TryFrom<u128> for NonZeroBigUint<M> {
    type Error = NonZeroError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Self::try_from(BigUint::from(value))
    }
}

impl<M: ManagedTypeApi> TryFrom<ManagedBuffer<M>> for NonZeroBigUint<M> {
    type Error = NonZeroError;

    fn try_from(item: ManagedBuffer<M>) -> Result<Self, Self::Error> {
        Self::try_from(BigUint::from(item))
    }
}

impl<M: ManagedTypeApi> TryFrom<&ManagedBuffer<M>> for NonZeroBigUint<M> {
    type Error = NonZeroError;

    fn try_from(item: &ManagedBuffer<M>) -> Result<Self, Self::Error> {
        Self::try_from(BigUint::from(item))
    }
}

impl<M: ManagedTypeApi> NonZeroBigUint<M> {
    pub(super) fn wrap_big_int_unchecked(value: BigInt<M>) -> Self {
        NonZeroBigUint { value }
    }

    pub(crate) unsafe fn new_unchecked(bu: BigUint<M>) -> Self {
        Self::wrap_big_int_unchecked(bu.value)
    }

    /// Will return either Some, with a non-zero value, or None, for zero.
    pub fn new(bu: BigUint<M>) -> Option<Self> {
        if bu == 0u32 {
            None
        } else {
            unsafe { Some(Self::new_unchecked(bu)) }
        }
    }

    /// Convenience constructor, which will signal error if the input is 0.
    pub fn new_or_panic(bu: BigUint<M>) -> Self {
        Self::new(bu).unwrap_or_else(|| quick_signal_error::<M>(err_msg::ZERO_VALUE_NOT_ALLOWED))
    }

    /// Drops the non-zero restriction.
    pub fn into_big_uint(self) -> BigUint<M> {
        BigUint { value: self.value }
    }

    /// Drops the non-zero restriction.
    pub fn as_big_uint(&self) -> &BigUint<M> {
        // safe because of #repr(transparent) on both sides
        // also because it is a loosening of the non-zero restriction
        unsafe { core::mem::transmute(self) }
    }

    /// Used in some operator definitions. Using it directly could violate invariant.
    pub(super) unsafe fn as_big_uint_mut(&mut self) -> &mut BigUint<M> {
        unsafe { core::mem::transmute(self) }
    }

    /// Drops the non-zero and positive restriction.
    pub fn as_big_int(&self) -> &BigInt<M> {
        &self.value
    }

    /// Drops the non-zero and positive restriction.
    pub fn into_big_int(self) -> BigInt<M> {
        self.value
    }
}

// TODO: figure out what type should be used in a non-managed context

impl<M> TypeAbiFrom<Self> for NonZeroBigUint<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Self> for NonZeroBigUint<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbi for NonZeroBigUint<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("NonZeroBigUint")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("NonZeroBigUint<$API>")
    }
}

impl<M: ManagedTypeApi> Clone for NonZeroBigUint<M> {
    fn clone(&self) -> Self {
        NonZeroBigUint {
            value: self.as_big_int().clone(),
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for NonZeroBigUint<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_big_uint().top_encode_or_handle_err(output, h)
    }
}

impl<M: ManagedTypeApi> NestedEncode for NonZeroBigUint<M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_big_uint().dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> NestedDecode for NonZeroBigUint<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let bu = BigUint::<M>::dep_decode_or_handle_err(input, h)?;
        Self::try_from(bu)
            .map_err(|_| h.handle_error(DecodeError::from(err_msg::ZERO_VALUE_NOT_ALLOWED)))
    }
}

impl<M: ManagedTypeApi> TopDecode for NonZeroBigUint<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let bu = BigUint::<M>::top_decode_or_handle_err(input, h)?;
        Self::try_from(bu)
            .map_err(|_| h.handle_error(DecodeError::from(err_msg::ZERO_VALUE_NOT_ALLOWED)))
    }
}

impl<M: ManagedTypeApi> SCDisplay for NonZeroBigUint<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        SCDisplay::fmt(self.as_big_uint(), f)
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for NonZeroBigUint<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NonZeroBigUint")
            .field("handle", &self.value.handle.clone())
            .field(
                "hex-value-be",
                &encode_bytes_as_hex(self.as_big_uint().to_bytes_be().as_slice()),
            )
            .finish()
    }
}
