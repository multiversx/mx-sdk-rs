use generic_array::typenum::U8;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    types::{
        ManagedVecItem, ManagedVecItemPayloadBuffer, NonZeroBigUint, Payment, PaymentRefs, Ref,
        TokenId, managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index,
    },
};

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug)]
pub struct FungiblePayment<M: ManagedTypeApi> {
    pub token_identifier: TokenId<M>,
    pub amount: NonZeroBigUint<M>,
}

impl<M: ManagedTypeApi> FungiblePayment<M> {
    pub fn new(token_identifier: TokenId<M>, amount: NonZeroBigUint<M>) -> Self {
        FungiblePayment {
            token_identifier,
            amount,
        }
    }

    pub fn into_payment(self) -> Payment<M> {
        Payment::new(self.token_identifier, 0, self.amount)
    }

    pub fn as_payment_refs(&self) -> PaymentRefs<'_, M> {
        PaymentRefs::new(&self.token_identifier, 0, &self.amount)
    }
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for FungiblePayment<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for FungiblePayment<M> {}

impl<M: ManagedTypeApi> TypeAbi for FungiblePayment<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "FungiblePayment".into()
    }

    fn type_name_rust() -> TypeName {
        "FungiblePayment<$API>".into()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for FungiblePayment<M> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<U8>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Ref<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            FungiblePayment {
                token_identifier: managed_vec_item_read_from_payload_index(payload, &mut index),
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
            managed_vec_item_save_to_payload_index(self.amount, payload, &mut index);
        }
    }
}
