multiversx_sc::imports!();

use core::marker::PhantomData;

use multiversx_sc::codec::Empty;

use super::merged_token_instances::MergedTokenInstances;

pub trait AllMergeScTraits = super::merged_token_setup::MergedTokenSetupModule
    + crate::default_issue_callbacks::DefaultIssueCallbacksModule
    + crate::pause::PauseModule;

pub trait MergedTokenAttributesCreator {
    type ScType: AllMergeScTraits;
    type AttributesType: TopEncode + TopDecode;

    fn get_merged_token_attributes(
        &self,
        sc: &Self::ScType,
        merged_token_id: &TokenIdentifier<<Self::ScType as ContractBase>::Api>,
        merged_token_raw_attributes: &MergedTokenInstances<<Self::ScType as ContractBase>::Api>,
    ) -> Self::AttributesType;
}

pub struct DefaultMergedAttributesWrapper<Sc: AllMergeScTraits> {
    _phantom: PhantomData<Sc>,
}

impl<Sc> DefaultMergedAttributesWrapper<Sc>
where
    Sc: AllMergeScTraits,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Sc> Default for DefaultMergedAttributesWrapper<Sc>
where
    Sc: AllMergeScTraits,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Sc> MergedTokenAttributesCreator for DefaultMergedAttributesWrapper<Sc>
where
    Sc: AllMergeScTraits,
{
    type ScType = Sc;
    type AttributesType = Empty;

    fn get_merged_token_attributes(
        &self,
        _sc: &Self::ScType,
        _merged_token_id: &TokenIdentifier<<Self::ScType as ContractBase>::Api>,
        _merged_token_raw_attributes: &MergedTokenInstances<<Self::ScType as ContractBase>::Api>,
    ) -> Self::AttributesType {
        Empty
    }
}
