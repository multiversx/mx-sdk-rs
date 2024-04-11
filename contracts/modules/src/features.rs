multiversx_sc::imports!();

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
#[multiversx_sc::module]
pub trait FeaturesModule {
    #[storage_mapper("feat:")]
    fn feature_flag(&self, feature_name: &FeatureName<Self::Api>) -> SingleValueMapper<u8>;

    fn check_feature_on(&self, feature_name: &'static [u8], default: bool) {
        let flag = self.feature_flag(&FeatureName(feature_name.into())).get();
        let value = match flag {
            FEATURE_NOT_SET => default,
            FEATURE_ON => true,
            _ => false,
        };
        require!(value, "{} currently disabled", feature_name);
    }

    #[only_owner]
    #[endpoint(setFeatureFlag)]
    fn set_feature_flag_endpoint(&self, feature_name: ManagedBuffer, value: bool) {
        let feature_value = if value { FEATURE_ON } else { FEATURE_OFF };
        self.feature_flag(&FeatureName(feature_name))
            .set(feature_value);
    }
}

multiversx_sc::derive_imports!();

#[derive(TopEncode)]
pub struct FeatureName<M>(ManagedBuffer<M>)
where
    M: ManagedTypeApi;

use multiversx_sc::codec::*;
impl<M> NestedEncode for FeatureName<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        dest.push_specialized((), &self.0, h)
    }
}
