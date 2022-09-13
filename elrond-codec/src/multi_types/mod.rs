mod multi_value_ignore;
mod multi_value_optional;
mod multi_value_placeholder;
mod multi_value_tuple;
mod multi_value_unit;

#[cfg(feature = "alloc")]
mod multi_value_vec;

pub use multi_value_ignore::IgnoreValue;
pub use multi_value_optional::OptionalValue;
pub use multi_value_placeholder::*;
pub use multi_value_tuple::*;

#[cfg(feature = "alloc")]
pub use multi_value_vec::MultiValueVec;
