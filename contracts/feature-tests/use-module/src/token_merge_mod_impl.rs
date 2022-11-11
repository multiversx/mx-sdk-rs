elrond_wasm::imports!();

use elrond_wasm_modules::token_merge::custom_merged_token_attributes::DefaultMergedAttributesWrapper;

#[elrond_wasm::module]
pub trait TokenMergeModImpl:
    elrond_wasm_modules::pause::PauseModule
    + elrond_wasm_modules::token_merge::TokenMergeModule
    + elrond_wasm_modules::token_merge::merged_token_setup::MergedTokenSetupModule
    + elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[payable("*")]
    #[endpoint(mergeTokens)]
    fn merge_tokens_endpoint(&self) -> EsdtTokenPayment {
        let payments = self.call_value().all_esdt_transfers();
        let attributes_creator = DefaultMergedAttributesWrapper::new();
        self.merge_tokens(payments, &attributes_creator)
    }

    #[payable("*")]
    #[endpoint(splitTokens)]
    fn split_tokens_endpoint(&self) -> ManagedVec<EsdtTokenPayment> {
        let payments = self.call_value().all_esdt_transfers();
        self.split_tokens(payments)
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
