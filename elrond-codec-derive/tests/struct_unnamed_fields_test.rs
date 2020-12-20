extern crate elrond_codec_derive;
use elrond_codec_derive::*;

use elrond_codec::test_util::{check_dep_encode_decode, check_top_encode_decode};
use elrond_codec::*;

// to test, run the following command in elrond-codec-derive folder:
// cargo expand --test struct_unnamed_fields_test > expanded.rs

#[derive(NestedEncode)]
pub struct StructWithUnnamedFields(u16, Vec<u8>);
