
#![no_std]
#![allow(clippy::string_lit_as_bytes)]

#[macro_use]
extern crate elrond_wasm;

imports!();

pub const FEATURE_NOT_SET: u8 = 0;
pub const FEATURE_ON: u8 = 1;
pub const FEATURE_OFF: u8 = 2;

/// Standard module for managing feature flags.
#[elrond_wasm_derive::module(FeaturesModuleImpl)]
pub trait FeaturesModule {

    #[storage_get("feat:")]
    fn get_feature_flag(&self, feature_name: FeatureName) -> u8;

    #[storage_set("feat:")]
    fn set_feature_flag(&self, feature_name: FeatureName, value: u8);

    fn check_feature_on(&self, feature_name: &'static [u8], default: bool) -> SCResult<()> {
        let flag = self.get_feature_flag(FeatureName(feature_name));
        let value = match flag {
            FEATURE_NOT_SET => default,
            FEATURE_ON => true,
            _ => false,
        };
        if value {
            Ok(())
        } else {
            let mut msg = feature_name.to_vec();
            msg.extend_from_slice(&b" currently disabled"[..]);
            SCResult::Err(msg.into())
        }
    }

    #[endpoint(setFeatureFlag)]
    fn set_feature_flag_endpoint(&self, feature_name: Vec<u8>, value: bool) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner_address(),
            "only owner allowed to change features");
        
        self.set_feature_flag(
            FeatureName(feature_name.as_slice()),
            if value { FEATURE_ON } else { FEATURE_OFF });
        Ok(())
    }

}

pub struct FeatureName<'a>(&'a [u8]);

use elrond_wasm::elrond_codec::*;
impl<'a> NestedEncode for FeatureName<'a> {
    #[inline]
    fn dep_encode_to<O: OutputBuffer>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.write(&self.0[..]);
        Result::Ok(())
    }
}

/// Expands to a snippet that returns with error if a feature is not enabled.
/// Also receives a default, which is the feature value if unset.
#[macro_export]
macro_rules! feature_guard {
    ($feature_module: expr, $feature_name:expr, $default:expr) => {
        sc_try!($feature_module.check_feature_on(&$feature_name[..], $default));
    }
}
