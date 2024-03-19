use core::ops::Add;

use crate::imports::ManagedVecItem;
struct PayloadData<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> PayloadData<N> {
    fn concat_slices(slice1: &[u8], slice2: &[u8]) -> [u8; N] {
        assert!(slice1.len() + slice2.len() == N, "result size not right");

        let mut result = [0u8; N];
        result[..slice1.len()].copy_from_slice(slice1);
        result[slice1.len()..].copy_from_slice(slice2);
        result
    }
}

pub trait Concat {
    
}

impl<const N: usize, const M: usize, P1: Payload, P2: Payload> ConcatPayloads<P1> for P2 {
    type Result;

    fn concat(self, other: P1) -> Self::Result {
        todo!()
    }
}
macro_rules! add_const_payloads {
    ($dec1:expr, $dec2:expr, $result_add:expr) => {
        impl Add<PayloadData<$dec2>> for PayloadData<$dec1> {
            type Output = PayloadData<$result_add>;
            fn add(self, _rhs: PayloadData<$dec2>) -> Self::Output {
                PayloadData::<$result_add> {
                    data: PayloadData::concat_slices(self.data(), _rhs.data()),
                }
            }
        }
    };
}

add_const_payloads!(1, 2, 3);

pub trait Payload {
    fn size(&self) -> usize;
    fn data(&self) -> &[u8];
}

impl<const N: usize> Payload for PayloadData<N> {
    fn size(&self) -> usize {
        N
    }

    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

pub trait ConcatPayloads<P: Payload> {
    type Result: Payload;

    fn concat(self, other: P) -> Self::Result;
}

impl<P1: Payload, P2: Payload> ConcatPayloads<P2> for P1 {
    type Result = PayloadData<N>;

    fn concat(self, other: P2) -> Self::Result {
        PayloadData::<N> { data: todo!() }
    }
}

impl<const N: usize> ManagedVecItem for PayloadData<N> {
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut byte_arr = [0u8; N];
        reader(&mut byte_arr[..]);
        PayloadData { data: byte_arr }
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        writer(&self.data)
    }

    const SKIPS_RESERIALIZATION: bool = false;

    type Ref<'a> = Self;

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    type PayloadContent = Self;

    fn payload_content(self) -> Self::PayloadContent {
        self
    }
}

// impl<T> ManagedVecItem for Option<T>
// where
//     T: ManagedVecItem + Payload,
// {
//     const SKIPS_RESERIALIZATION: bool = false;

//     type Ref<'a> = Self;

//     fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
//         todo!()
//     }

//     unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
//         reader: Reader,
//     ) -> Self::Ref<'a> {
//         todo!()
//     }

//     fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
//         todo!()
//     }

//     type PayloadContent = T;
// }

// //     const PAYLOAD: dyn Payload = T::PAYLOAD;
// // }
