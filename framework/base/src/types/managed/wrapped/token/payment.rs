use generic_array::typenum::U16;
use multiversx_sc_codec::IntoMultiValue;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{ErrorApiImpl, ManagedTypeApi},
    codec::{
        self, NestedDecode, TopDecode,
        derive::{NestedEncode, TopEncode},
    },
    err_msg,
    types::{
        BigUint, Egld, EgldOrEsdtTokenPayment, EsdtTokenPayment, EsdtTokenPaymentRefs,
        FungiblePayment, ManagedVecItem, ManagedVecItemPayloadBuffer, NonZeroBigUint,
        PaymentMultiValue, PaymentRefs, Ref, TokenId, managed_vec_item_read_from_payload_index,
        managed_vec_item_save_to_payload_index,
    },
};

/// Represents a payment in a specific token with a guaranteed non-zero amount.
///
/// A `Payment` encapsulates all the information needed to represent a token transfer:
/// - The token identifier (native EGLD or ESDT token)
/// - The token nonce (0 for fungible tokens, > 0 for NFTs/SFTs)
/// - The amount (guaranteed to be non-zero)
///
/// This type is commonly used in smart contracts for handling incoming payments,
/// storing payment information, and ensuring payment validity through the type system.
///
/// # Key Features
///
/// - **Type Safety**: The amount is guaranteed to be non-zero through [`NonZeroBigUint`]
/// - **Universal Token Support**: Works with both native EGLD and ESDT tokens
/// - **NFT/SFT Support**: Handles both fungible (nonce=0) and non-fungible tokens (nonce>0)
/// - **Serialization**: Supports encoding/decoding for blockchain storage and communication
///
/// # Examples
///
/// ```rust
/// # use multiversx_sc::types::*;
/// # use multiversx_sc::api::ManagedTypeApi;
/// # fn example<M: ManagedTypeApi>() -> Result<(Payment<M>, Payment<M>), NonZeroError> {
/// // Create a fungible token payment
/// let usdc_payment = Payment::new(
///     TokenId::from("USDC-123456"),
///     0,
///     NonZeroBigUint::try_from(1000u64)?,
/// );
///
/// // Create an NFT payment
/// let nft_payment = Payment::try_new(
///     "NFT-456789",
///     42,
///     1u64
/// )?;
///
/// // Check if payment is fungible
/// assert!(usdc_payment.is_fungible());
/// assert!(!nft_payment.is_fungible());
/// # Ok((usdc_payment, nft_payment))
/// # }
/// ```
///
/// # See Also
///
/// - [`FungiblePayment`] for fungible-only payments
/// - [`NonZeroBigUint`] for the amount type constraints
#[derive(TopEncode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub struct Payment<M: ManagedTypeApi> {
    /// The token identifier (native EGLD or ESDT token)
    pub token_identifier: TokenId<M>,
    /// The token nonce (0 for fungible tokens, > 0 for NFTs/SFTs)
    pub token_nonce: u64,
    /// The payment amount (guaranteed to be non-zero)
    pub amount: NonZeroBigUint<M>,
}

impl<M: ManagedTypeApi> Payment<M> {
    /// Creates a new `Payment` with the specified token identifier, nonce, and amount.
    ///
    /// This constructor accepts any type that can be converted into a `TokenId<M>` and any type that can be converted into a `NonZeroBigUint<M>`, for ergonomic usage.
    ///
    /// # Arguments
    ///
    /// * `token_identifier` - Any type convertible to `TokenId<M>` (e.g., `&str`, `String`, `ManagedBuffer<M>`, etc.)
    /// * `token_nonce` - The token nonce (0 for fungible tokens, > 0 for NFTs/SFTs)
    /// * `amount` - Any type convertible to `NonZeroBigUint<M>` (e.g., `NonZeroU64`, `u64`, `u128`, etc.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use multiversx_sc::types::*;
    /// # use multiversx_sc::api::ManagedTypeApi;
    /// # fn example<M: ManagedTypeApi>() -> (Payment<M>, Payment<M>) {
    /// let token_id = TokenId::from("USDC-123456");
    /// let amount = NonZeroBigUint::try_from(1000u64).unwrap();
    ///
    /// // Create a fungible token payment
    /// let payment = Payment::new(token_id, 0, amount);
    ///
    /// // Create an NFT payment
    /// let nft_id = TokenId::from("NFT-456789");
    /// let nft_amount = NonZeroBigUint::try_from(1u64).unwrap();
    /// let nft_payment = Payment::new(nft_id, 42, nft_amount);
    /// # (payment, nft_payment)
    /// # }
    /// ```
    pub fn new<T, A>(token_identifier: T, token_nonce: u64, amount: A) -> Self
    where
        T: Into<TokenId<M>>,
        A: Into<NonZeroBigUint<M>>,
    {
        Payment {
            token_identifier: token_identifier.into(),
            token_nonce,
            amount: amount.into(),
        }
    }

    /// Attempts to create a new `Payment` from flexible input types.
    ///
    /// This is a more flexible version of [`Payment::new`] that accepts arguments
    /// implementing `Into` and `TryInto` traits, allowing for convenient conversion
    /// from various compatible types.
    ///
    /// # Arguments
    ///
    /// * `token_identifier` - Any type that can be converted into a `TokenId<M>`
    ///   (e.g., `ManagedBuffer<M>`, `&str`, `String`, etc.)
    /// * `token_nonce` - The token nonce (0 for fungible tokens, > 0 for NFTs/SFTs)
    /// * `amount` - Any type that can be converted into a `NonZeroBigUint<M>`
    ///   (e.g., `BigUint<M>`, `u128`, `ManagedBuffer<M>`, etc.)
    ///
    /// # Returns
    ///
    /// * `Ok(Payment)` - If all conversions succeed and the amount is non-zero
    /// * `Err(NonZeroError)` - If the amount converts to zero
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use multiversx_sc::types::*;
    /// # use multiversx_sc::api::ManagedTypeApi;
    /// # fn example<M: ManagedTypeApi>() -> Result<(), NonZeroError> {
    /// let token_id = TokenId::<M>::from("TOKEN-123456");
    /// let amount = BigUint::from(1000u64);
    ///
    /// // Create payment from BigUint (might be zero)
    /// let payment = Payment::<M>::try_new(token_id, 0, amount)?;
    ///
    /// // Create payment from u128
    /// let payment = Payment::<M>::try_new("USDC-123456", 0, 500u128)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_new<T, A, E>(token_identifier: T, token_nonce: u64, amount: A) -> Result<Self, E>
    where
        T: Into<TokenId<M>>,
        A: TryInto<NonZeroBigUint<M>, Error = E>,
    {
        Ok(Payment {
            token_identifier: token_identifier.into(),
            token_nonce,
            amount: amount.try_into()?,
        })
    }

    pub fn is_fungible(&self) -> bool {
        self.token_nonce == 0
    }

    pub fn fungible_or_panic(self) -> FungiblePayment<M> {
        if !self.is_fungible() {
            M::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED.as_bytes());
        }
        FungiblePayment::new(self.token_identifier, self.amount)
    }

    #[inline]
    pub fn into_tuple(self) -> (TokenId<M>, u64, NonZeroBigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }

    #[inline]
    pub fn as_tuple(&self) -> (&TokenId<M>, u64, &NonZeroBigUint<M>) {
        (&self.token_identifier, self.token_nonce, &self.amount)
    }

    /// Zero-cost conversion to the legacy payment type, `EgldOrEsdtTokenPayment`.
    pub fn into_egld_or_esdt_payment(self) -> EgldOrEsdtTokenPayment<M> {
        EgldOrEsdtTokenPayment {
            token_identifier: self.token_identifier.into_legacy(),
            token_nonce: self.token_nonce,
            amount: self.amount.into_big_uint(),
        }
    }

    /// Same as `map_egld_or_esdt`, but only takes a reference,
    /// and consequently, the closures also only get references.
    pub fn map_ref_egld_or_esdt<Context, D, F, U>(
        &self,
        context: Context,
        for_egld: D,
        for_esdt: F,
    ) -> U
    where
        D: FnOnce(Context, Egld<&BigUint<M>>) -> U,
        F: FnOnce(Context, EsdtTokenPaymentRefs<'_, M>) -> U,
    {
        self.as_refs().map_egld_or_esdt(context, for_egld, for_esdt)
    }

    pub fn map_egld_or_esdt<Context, D, F, U>(self, context: Context, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce(Context, Egld<BigUint<M>>) -> U,
        F: FnOnce(Context, EsdtTokenPayment<M>) -> U,
    {
        if self.token_identifier.is_native() {
            for_egld(context, Egld(self.amount.into_big_uint()))
        } else {
            for_esdt(
                context,
                EsdtTokenPayment::new(
                    unsafe { self.token_identifier.into_esdt_unchecked() },
                    self.token_nonce,
                    self.amount.into_big_uint(),
                ),
            )
        }
    }

    pub fn as_refs(&self) -> PaymentRefs<'_, M> {
        PaymentRefs::new(&self.token_identifier, self.token_nonce, &self.amount)
    }
}

impl<M> AsRef<Payment<M>> for &Payment<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn as_ref(&self) -> &Payment<M> {
        self
    }
}

impl<M: ManagedTypeApi, T, A> From<(T, u64, A)> for Payment<M>
where
    T: Into<TokenId<M>>,
    A: Into<NonZeroBigUint<M>>,
{
    #[inline]
    fn from(value: (T, u64, A)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier.into(), token_nonce, amount.into())
    }
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for Payment<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for Payment<M> {}

impl<M: ManagedTypeApi> TypeAbi for Payment<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "Payment".into()
    }

    fn type_name_rust() -> TypeName {
        "Payment<$API>".into()
    }
}

impl<M: ManagedTypeApi> TopDecode for Payment<M> {
    fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::TopDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let mut nested_buffer = top_input.into_nested_buffer();
        let result = Self::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        if !codec::NestedDecodeInput::is_depleted(&nested_buffer) {
            return Err(h.handle_error(codec::DecodeError::INPUT_TOO_LONG));
        }
        Ok(result)
    }
}

impl<M: ManagedTypeApi> NestedDecode for Payment<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Ok(Payment {
            token_identifier: TokenId::<M>::dep_decode_or_handle_err(input, h)?,
            token_nonce: <u64>::dep_decode_or_handle_err(input, h)?,
            amount: NonZeroBigUint::<M>::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl<M: ManagedTypeApi> IntoMultiValue for Payment<M> {
    type MultiValue = PaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for Payment<M> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<U16>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Ref<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            Payment {
                token_identifier: managed_vec_item_read_from_payload_index(payload, &mut index),
                token_nonce: managed_vec_item_read_from_payload_index(payload, &mut index),
                amount: managed_vec_item_read_from_payload_index(payload, &mut index),
            }
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        unsafe { Ref::new(Self::read_from_payload(payload)) }
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let mut index = 0;

        unsafe {
            managed_vec_item_save_to_payload_index(self.token_identifier, payload, &mut index);
            managed_vec_item_save_to_payload_index(self.token_nonce, payload, &mut index);
            managed_vec_item_save_to_payload_index(self.amount, payload, &mut index);
        }
    }
}
