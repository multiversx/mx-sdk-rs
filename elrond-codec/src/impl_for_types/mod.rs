mod impl_array;
mod impl_array_vec;
mod impl_bool;
mod impl_bytes;
pub mod impl_empty;
mod impl_non_zero_usize;
mod impl_num_signed;
mod impl_num_unsigned;
mod impl_option;
mod impl_phantom;
mod impl_ref;
mod impl_slice;
mod impl_string;
mod impl_tuple;
mod impl_unit;
mod impl_vec;
mod local_macro;

#[cfg(feature = "num-bigint")]
mod impl_rust_big_int;

#[cfg(feature = "num-bigint")]
mod impl_rust_big_uint;
