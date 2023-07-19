multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use core::marker::PhantomData;

use multiversx_sc_modules::token_merge::{
    custom_merged_token_attributes::{
        AllMergeScTraits, DefaultMergedAttributesWrapper, MergedTokenAttributesCreator,
    },
    merged_token_instances::MergedTokenInstances,
};

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct CustomAttributes {
    pub first: u32,
    pub second: u64,
}

#[multiversx_sc::module]
pub trait TokenMergeModImpl:
    multiversx_sc_modules::pause::PauseModule
    + multiversx_sc_modules::token_merge::TokenMergeModule
    + multiversx_sc_modules::token_merge::merged_token_setup::MergedTokenSetupModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[payable("*")]
    #[endpoint(mergeTokens)]
    fn merge_tokens_endpoint(&self) -> EsdtTokenPayment {
        let payments = self.call_value().all_esdt_transfers();
        let attributes_creator = DefaultMergedAttributesWrapper::new();
        self.merge_tokens(&*payments, &attributes_creator)
    }

    #[payable("*")]
    #[endpoint(mergeTokensCustomAttributes)]
    fn merge_tokens_custom_attributes_endpoint(&self) -> EsdtTokenPayment {
        let payments = self.call_value().all_esdt_transfers();
        let attributes_creator = CustomMergedAttributesWrapper::new();
        self.merge_tokens(&*payments, &attributes_creator)
    }

    #[payable("*")]
    #[endpoint(splitTokens)]
    fn split_tokens_endpoint(&self) -> ManagedVec<EsdtTokenPayment> {
        let payments = self.call_value().all_esdt_transfers();
        self.split_tokens(&*payments)
    }

    #[payable("*")]
    #[endpoint(splitTokenPartial)]
    fn split_token_partial_endpoint(
        &self,
        tokens_to_remove: ManagedVec<EsdtTokenPayment>,
    ) -> ManagedVec<EsdtTokenPayment> {
        let payment = self.call_value().single_esdt();
        let attributes_creator = DefaultMergedAttributesWrapper::new();
        self.split_token_partial(payment, tokens_to_remove, &attributes_creator)
    }
}

pub struct CustomMergedAttributesWrapper<Sc: AllMergeScTraits> {
    _phantom: PhantomData<Sc>,
}

impl<Sc> CustomMergedAttributesWrapper<Sc>
where
    Sc: AllMergeScTraits,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Sc> Default for CustomMergedAttributesWrapper<Sc>
where
    Sc: AllMergeScTraits,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Sc> MergedTokenAttributesCreator for CustomMergedAttributesWrapper<Sc>
where
    Sc: AllMergeScTraits,
{
    type ScType = Sc;
    type AttributesType = CustomAttributes;

    fn get_merged_token_attributes(
        &self,
        _sc: &Self::ScType,
        _merged_token_id: &TokenIdentifier<<Self::ScType as ContractBase>::Api>,
        _merged_token_raw_attributes: &MergedTokenInstances<<Self::ScType as ContractBase>::Api>,
    ) -> Self::AttributesType {
        CustomAttributes {
            first: 5u32,
            second: 10u64,
        }
    }
}
