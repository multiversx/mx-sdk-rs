elrond_wasm::imports!();

pub const FEATURE_NOT_SET: u8 = 0;
pub const FEATURE_ON: u8 = 1;
pub const FEATURE_OFF: u8 = 2;

/// This is a standard smart contract module, that when added to a smart contract offers feature flag capabilities.
///
/// It offers:
/// * an endpoint where the owner can turn features on/off
/// * a method to check if feature is on or not
/// * a macro to make calling this method even more compact
///
#[elrond_wasm::module]
pub trait FeaturesModule {
    #[storage_get("feat:")]
    fn get_feature_flag(&self, feature_name: FeatureName) -> u8;

    #[storage_set("feat:")]
    fn set_feature_flag(&self, feature_name: FeatureName, value: u8);

    fn check_feature_on(&self, feature_name: &'static [u8], default: bool) {
        let flag = self.get_feature_flag(FeatureName(feature_name));
        let value = match flag {
            FEATURE_NOT_SET => default,
            FEATURE_ON => true,
            _ => false,
        };
        if !value {
            let mut err = self.error().new_error();
            err.append_bytes(feature_name);
            err.append_bytes(&b" currently disabled"[..]);
            err.exit_now()
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
