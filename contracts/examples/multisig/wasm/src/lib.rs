////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    multisig
    (
        deposit
        discardAction
        getActionLastIndex
        getNumBoardMembers
        getNumProposers
        getQuorum
        performAction
        proposeAddBoardMember
        proposeAddProposer
        proposeAsyncCall
        proposeChangeQuorum
        proposeRemoveUser
        proposeSCDeployFromSource
        proposeSCUpgradeFromSource
        proposeTransferExecute
        quorumReached
        sign
        signed
        unsign
    )
}

elrond_wasm_node::wasm_empty_callback! {}
