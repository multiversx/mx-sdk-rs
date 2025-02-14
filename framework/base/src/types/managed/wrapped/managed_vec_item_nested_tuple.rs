use generic_array::typenum::U1;

use super::{
    ManagedVecItem, ManagedVecItemEmptyPayload, ManagedVecItemPayload, ManagedVecItemPayloadAdd,
    ManagedVecItemPayloadBuffer, ManagedVecItemPayloadMax,
};

/// Syntactic sugar, that allows us to more easily represent struct payloads as nested tuples.
pub trait ManagedVecItemStructPayloadTuple {
    type StructPayload: ManagedVecItemPayload;
}

/// Syntactic sugar, it allows us to get the maximum payload length in a list at compile time.
pub trait ManagedVecItemMaxPayloadTuple {
    type MaxPayload: ManagedVecItemPayload;
}

/// End of the list.
impl ManagedVecItemStructPayloadTuple for () {
    type StructPayload = ManagedVecItemEmptyPayload;
}

/// End of the list.
impl ManagedVecItemMaxPayloadTuple for () {
    type MaxPayload = ManagedVecItemEmptyPayload; // for the discriminant
}

impl<Head, Tail> ManagedVecItemStructPayloadTuple for (Head, Tail)
where
    Head: ManagedVecItem,
    Tail: ManagedVecItemStructPayloadTuple,
    Head::PAYLOAD: ManagedVecItemPayloadAdd<Tail::StructPayload>,
{
    type StructPayload = <Head::PAYLOAD as ManagedVecItemPayloadAdd<Tail::StructPayload>>::Output;
}

impl<Head, Tail> ManagedVecItemMaxPayloadTuple for (Head, Tail)
where
    Head: ManagedVecItem,
    Tail: ManagedVecItemStructPayloadTuple,
    Head::PAYLOAD: ManagedVecItemPayloadMax<Tail::StructPayload>,
{
    type MaxPayload = <Head::PAYLOAD as ManagedVecItemPayloadMax<Tail::StructPayload>>::Max;
}

/// Syntactic sugar, that allows us to more easily represent enum payloads as nested tuples.
///
/// It is always the maximum payload length + 1.
pub trait ManagedVecItemEnumPayloadTuple {
    type EnumPayload: ManagedVecItemPayload;
}

impl<T> ManagedVecItemEnumPayloadTuple for T
where
    T: ManagedVecItemMaxPayloadTuple,
    T::MaxPayload: ManagedVecItemPayloadAdd<ManagedVecItemPayloadBuffer<U1>>,
{
    type EnumPayload =
        <T::MaxPayload as ManagedVecItemPayloadAdd<ManagedVecItemPayloadBuffer<U1>>>::Output;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn managed_vec_item_nested_tuple_struct_test() {
        assert_struct_payload_size::<()>(0);
        assert_struct_payload_size::<(u8, ())>(1);
        assert_struct_payload_size::<(usize, ())>(4);
        assert_struct_payload_size::<(usize, (usize, ()))>(8);
        assert_struct_payload_size::<(Option<usize>, ())>(5);
    }

    fn assert_struct_payload_size<N: ManagedVecItemStructPayloadTuple>(expected_size: usize) {
        assert_eq!(N::StructPayload::payload_size(), expected_size);
    }

    #[test]
    fn managed_vec_item_nested_tuple_enum_test() {
        assert_enum_payload_size::<()>(1);
        assert_enum_payload_size::<(u8, ())>(2);
        assert_enum_payload_size::<(u32, (i64, ()))>(9);
        assert_enum_payload_size::<(usize, ())>(5);
        assert_enum_payload_size::<(usize, (usize, ()))>(5);
        assert_enum_payload_size::<(Option<usize>, ())>(6);
    }

    fn assert_enum_payload_size<N: ManagedVecItemEnumPayloadTuple>(expected_size: usize) {
        assert_eq!(N::EnumPayload::payload_size(), expected_size);
    }
}
