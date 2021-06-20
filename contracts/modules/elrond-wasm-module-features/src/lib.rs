#![no_std]

elrond_wasm::imports!();

pub const FEATURE_NOT_SET: u8 = 0;
pub const FEATURE_ON: u8 = 1;
pub const FEATURE_OFF: u8 = 2;

/// Standard module for managing feature flags.
#[elrond_wasm_derive::module]
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
		require!(
			self.blockchain().get_caller() == self.blockchain().get_owner_address(),
			"only owner allowed to change features"
		);

		self.set_feature_flag(
			FeatureName(feature_name.as_slice()),
			if value { FEATURE_ON } else { FEATURE_OFF },
		);
		Ok(())
	}
}

elrond_wasm::derive_imports!();

#[derive(TopEncode)]
pub struct FeatureName<'a>(&'a [u8]);

use elrond_wasm::elrond_codec::*;
impl<'a> NestedEncode for FeatureName<'a> {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		dest.write(self.0);
		Result::Ok(())
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		_: ExitCtx,
		_: fn(ExitCtx, EncodeError) -> !,
	) {
		dest.write(self.0);
	}
}

/// Expands to a snippet that returns with error if a feature is not enabled.
/// Also receives a default, which is the feature value if unset.
#[macro_export]
macro_rules! feature_guard {
	($feature_module: expr, $feature_name:expr, $default:expr) => {
		$feature_module.check_feature_on(&$feature_name[..], $default)?;
	};
}
