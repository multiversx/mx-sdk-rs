////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    use_module::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    use_module::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn call_mod_a() {
    use_module::endpoints::call_mod_a(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn call_mod_b() {
    use_module::endpoints::call_mod_b(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn call_mod_c() {
    use_module::endpoints::call_mod_c(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn cancel() {
    use_module::endpoints::cancel(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn changeLockTimeAfterVotingEndsInBlocks() {
    use_module::endpoints::changeLockTimeAfterVotingEndsInBlocks(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn changeMaxActionsPerProposal() {
    use_module::endpoints::changeMaxActionsPerProposal(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn changeMinTokenBalanceForProposing() {
    use_module::endpoints::changeMinTokenBalanceForProposing(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn changeQuorum() {
    use_module::endpoints::changeQuorum(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn changeVotingDelayInBlocks() {
    use_module::endpoints::changeVotingDelayInBlocks(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn changeVotingPeriodInBlocks() {
    use_module::endpoints::changeVotingPeriodInBlocks(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn checkFeatureGuard() {
    use_module::endpoints::checkFeatureGuard(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn checkPause() {
    use_module::endpoints::checkPause(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn depositTokensForAction() {
    use_module::endpoints::depositTokensForAction(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn dnsRegister() {
    use_module::endpoints::dnsRegister(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn downvote() {
    use_module::endpoints::downvote(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn execute() {
    use_module::endpoints::execute(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getGovernanceTokenId() {
    use_module::endpoints::getGovernanceTokenId(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getLockTimeAfterVotingEndsInBlocks() {
    use_module::endpoints::getLockTimeAfterVotingEndsInBlocks(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getMaxActionsPerProposal() {
    use_module::endpoints::getMaxActionsPerProposal(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getMinTokenBalanceForProposing() {
    use_module::endpoints::getMinTokenBalanceForProposing(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getProposalActions() {
    use_module::endpoints::getProposalActions(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getProposalDescription() {
    use_module::endpoints::getProposalDescription(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getProposalStatus() {
    use_module::endpoints::getProposalStatus(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getProposer() {
    use_module::endpoints::getProposer(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getQuorum() {
    use_module::endpoints::getQuorum(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getTotalDownvotes() {
    use_module::endpoints::getTotalDownvotes(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getTotalVotes() {
    use_module::endpoints::getTotalVotes(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getVotingDelayInBlocks() {
    use_module::endpoints::getVotingDelayInBlocks(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getVotingPeriodInBlocks() {
    use_module::endpoints::getVotingPeriodInBlocks(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn initGovernanceModule() {
    use_module::endpoints::initGovernanceModule(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn isPaused() {
    use_module::endpoints::isPaused(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn issueToken() {
    use_module::endpoints::issueToken(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn pause() {
    use_module::endpoints::pause(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn propose() {
    use_module::endpoints::propose(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn queue() {
    use_module::endpoints::queue(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setFeatureFlag() {
    use_module::endpoints::setFeatureFlag(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    use_module::endpoints::setLocalRoles(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn unpause() {
    use_module::endpoints::unpause(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn vote() {
    use_module::endpoints::vote(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn withdrawGovernanceTokens() {
    use_module::endpoints::withdrawGovernanceTokens(elrond_wasm_node::arwen_api());
}
