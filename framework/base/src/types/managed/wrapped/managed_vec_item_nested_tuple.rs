use super::{
    ManagedVecItem, ManagedVecItemEmptyPayload, ManagedVecItemPayload, ManagedVecItemPayloadAdd,
};

/// Syntactic sugar, that allows us to more easily represent composite payloads as nested tuples.
pub trait ManagedVecItemNestedTuple {
    type PAYLOAD: ManagedVecItemPayload;
    type Split1: ManagedVecItemPayload;
    type Split2: ManagedVecItemPayload;

    fn split_payload(payload: &Self::PAYLOAD) -> (&Self::Split1, &Self::Split2);
}

pub trait ManagedVecItemNestedTupleSplit<'a>: ManagedVecItemNestedTuple {
    type S;

    fn split_all(payload: &'a Self::PAYLOAD) -> Self::S;
}

/// End of the list.
impl ManagedVecItemNestedTuple for () {
    type PAYLOAD = ManagedVecItemEmptyPayload;
    type Split1 = ManagedVecItemEmptyPayload;
    type Split2 = ManagedVecItemEmptyPayload;

    fn split_payload(_payload: &Self::PAYLOAD) -> (&Self::Split1, &Self::Split2) {
        (&ManagedVecItemEmptyPayload, &ManagedVecItemEmptyPayload)
    }
}

impl<'a> ManagedVecItemNestedTupleSplit<'a> for () {
    type S = ();

    fn split_all(_payload: &'a Self::PAYLOAD) -> Self::S {
        ()
    }
}

impl<Head, Tail> ManagedVecItemNestedTuple for (Head, Tail)
where
    Head: ManagedVecItem,
    Tail: ManagedVecItemNestedTuple,
    Head::PAYLOAD: ManagedVecItemPayloadAdd<Tail::PAYLOAD>,
{
    type PAYLOAD = <Head::PAYLOAD as ManagedVecItemPayloadAdd<Tail::PAYLOAD>>::Output;
    type Split1 = <Head as ManagedVecItem>::PAYLOAD;
    type Split2 = <Tail as ManagedVecItemNestedTuple>::PAYLOAD;

    fn split_payload(payload: &Self::PAYLOAD) -> (&Self::Split1, &Self::Split2) {
        Head::PAYLOAD::split_from_add(payload)
    }
}

impl<'a, Head, Tail> ManagedVecItemNestedTupleSplit<'a> for (Head, Tail)
where
    Head: ManagedVecItem,
    Tail: ManagedVecItemNestedTupleSplit<'a>,
    Head::PAYLOAD: ManagedVecItemPayloadAdd<Tail::PAYLOAD>,
    Tail::PAYLOAD: 'a,
{
    type S = (&'a Head::PAYLOAD, Tail::S);

    fn split_all(payload: &'a Self::PAYLOAD) -> Self::S {
        let (hp, tp) = Head::PAYLOAD::split_from_add(payload);
        (hp, Tail::split_all(tp))
    }
}

// pub fn split_payload<Head, Tail>(
//     payload: &<(Head, Tail) as ManagedVecItemNestedTuple>::PAYLOAD,
// ) -> (&Head::PAYLOAD, &Tail::PAYLOAD)
// where
//     Head: ManagedVecItem,
//     Tail: ManagedVecItemNestedTuple,
//     Head::PAYLOAD: ManagedVecItemPayloadAdd<Tail::PAYLOAD>,
//     // (Head, Tail): ManagedVecItemNestedTuple,
// {
//     <Head::PAYLOAD as ManagedVecItemPayloadAdd<Tail::PAYLOAD>>::split_from_add(payload)
//     // <(Head, Tail) as ManagedVecItemNestedTuple>::PAYLOAD as
//     // unsafe {
//     //     let ptr1 = payload.buffer.as_ptr();
//     //     let ptr2 = ptr1.offset($dec1 as isize);
//     //     (core::mem::transmute(ptr1), core::mem::transmute(ptr2))
//     // }
// }

#[cfg(test)]
pub mod tests {
    use crate::types::ManagedVecItemPayloadBuffer;

    use super::*;

    #[test]
    fn managed_vec_item_nesteds_tuple_test() {
        assert_payload_size::<()>(0);
        assert_payload_size::<(u8, ())>(1);
        assert_payload_size::<(usize, ())>(4);
        assert_payload_size::<(usize, (usize, ()))>(8);
        assert_payload_size::<(Option<usize>, ())>(5);
    }

    fn assert_payload_size<N: ManagedVecItemNestedTuple>(expected_size: usize) {
        assert_eq!(N::PAYLOAD::payload_size(), expected_size);
    }

    #[test]
    fn split_all_test() {
        let p = ManagedVecItemPayloadBuffer::new_buffer();
        let (p1, (p2, ())) = <(u16, (u32, ()))>::split_all(&p);
    }

    // fn split_all_t<T>()
    // where
    //     T: ManagedVecItem,
    //     ManagedVecItemPayloadBuffer<1>: ManagedVecItemPayloadAdd<T::PAYLOAD>,
    //     (u16, (T, ())): ManagedVecItemNestedTuple,
    // {
    //     let p = ManagedVecItemPayloadBuffer::new_buffer();
    //     let (p1, (p2, ())) = <(u16, (T, ()))>::split_all(&p);
    // }
}
