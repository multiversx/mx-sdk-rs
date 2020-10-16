use alloc::vec::Vec;
use alloc::boxed::Box;

pub fn boxed_slice_into_vec<T: Clone>(bs: Box<[T]>) -> Vec<T> {
    bs.to_vec()
}

pub fn vec_into_boxed_slice<T>(v: Vec<T>) -> Box<[T]> {
    v.into_boxed_slice()
}
