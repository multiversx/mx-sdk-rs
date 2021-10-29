////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    multisig::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getQuorum() {
    multisig::endpoints::getQuorum(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getNumBoardMembers() {
    multisig::endpoints::getNumBoardMembers(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getNumProposers() {
    multisig::endpoints::getNumProposers(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getActionLastIndex() {
    multisig::endpoints::getActionLastIndex(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getActionData() {
    multisig::endpoints::getActionData(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn deposit() {
    multisig::endpoints::deposit(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getPendingActionFullInfo() {
    multisig::endpoints::getPendingActionFullInfo(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeAddBoardMember() {
    multisig::endpoints::proposeAddBoardMember(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeAddProposer() {
    multisig::endpoints::proposeAddProposer(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeRemoveUser() {
    multisig::endpoints::proposeRemoveUser(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeChangeQuorum() {
    multisig::endpoints::proposeChangeQuorum(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeSendEgld() {
    multisig::endpoints::proposeSendEgld(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeSCDeploy() {
    multisig::endpoints::proposeSCDeploy(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeSCDeployFromSource() {
    multisig::endpoints::proposeSCDeployFromSource(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeSCUpgrade() {
    multisig::endpoints::proposeSCUpgrade(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeSCUpgradeFromSource() {
    multisig::endpoints::proposeSCUpgradeFromSource(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn proposeAsyncCall() {
    multisig::endpoints::proposeAsyncCall(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn signed() {
    multisig::endpoints::signed(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn userRole() {
    multisig::endpoints::userRole(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getAllBoardMembers() {
    multisig::endpoints::getAllBoardMembers(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getAllProposers() {
    multisig::endpoints::getAllProposers(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn sign() {
    multisig::endpoints::sign(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn unsign() {
    multisig::endpoints::unsign(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getActionSigners() {
    multisig::endpoints::getActionSigners(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getActionSignerCount() {
    multisig::endpoints::getActionSignerCount(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getActionValidSignerCount() {
    multisig::endpoints::getActionValidSignerCount(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn quorumReached() {
    multisig::endpoints::quorumReached(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn performAction() {
    multisig::endpoints::performAction(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn discardAction() {
    multisig::endpoints::discardAction(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    multisig::endpoints::callBack(elrond_wasm_node::arwen_api());
}
