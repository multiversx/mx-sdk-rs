use crate::action::Action;

elrond_wasm::imports!();

fn get_code_metadata(upgradeable: bool, payable: bool, readable: bool) -> CodeMetadata {
    let mut code_metadata = CodeMetadata::DEFAULT;
    if upgradeable {
        code_metadata |= CodeMetadata::UPGRADEABLE;
    }
    if payable {
        code_metadata |= CodeMetadata::PAYABLE;
    }
    if readable {
        code_metadata |= CodeMetadata::READABLE;
    }
    code_metadata
}

/// Contains all events that can be emitted by the contract.
#[elrond_wasm::module]
pub trait MultisigProposeModule: crate::multisig_state::MultisigStateModule {
    fn propose_action(&self, action: Action<Self::Api>) -> SCResult<usize> {
        let caller_address = self.blockchain().get_caller();
        let caller_id = self.user_mapper().get_user_id(&caller_address);
        let caller_role = self.get_user_id_to_role(caller_id);
        require!(
            caller_role.can_propose(),
            "only board members and proposers can propose"
        );

        let action_id = self.action_mapper().push(&action);
        if caller_role.can_sign() {
            // also sign
            // since the action is newly created, the caller can be the only signer
            self.action_signer_ids(action_id).insert(caller_id);
        }

        Ok(action_id)
    }

    /// Initiates board member addition process.
    /// Can also be used to promote a proposer to board member.
    #[endpoint(proposeAddBoardMember)]
    fn propose_add_board_member(&self, board_member_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::AddBoardMember(board_member_address))
    }

    /// Initiates proposer addition process..
    /// Can also be used to demote a board member to proposer.
    #[endpoint(proposeAddProposer)]
    fn propose_add_proposer(&self, proposer_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::AddProposer(proposer_address))
    }

    /// Removes user regardless of whether it is a board member or proposer.
    #[endpoint(proposeRemoveUser)]
    fn propose_remove_user(&self, user_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::RemoveUser(user_address))
    }

    #[endpoint(proposeChangeQuorum)]
    fn propose_change_quorum(&self, new_quorum: usize) -> SCResult<usize> {
        self.propose_action(Action::ChangeQuorum(new_quorum))
    }

    /// Propose a transaction in which the contract can send EGLD
    /// and optionally execute a contract endpoint or builtin function.
    #[endpoint(proposeSendEgld)]
    fn propose_send_egld(
        &self,
        to: ManagedAddress,
        amount: BigUint,
        #[var_args] opt_function: OptionalArg<ManagedBuffer>,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let function_name = match opt_function {
            OptionalArg::Some(data) => data,
            OptionalArg::None => ManagedBuffer::new(),
        };
        self.propose_action(Action::SendEGLD {
            to,
            amount,
            function_name,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSendEsdt)]
    fn propose_send_esdt(
        &self,
        to: ManagedAddress,
        esdt_payment_args: ManagedCountedVarArgs<EsdtTokenPaymentMultiArg<Self::Api>>,
        #[var_args] opt_function: OptionalArg<ManagedBuffer>,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let mut esdt_payments_vec = ManagedVec::new();
        for payment_args in esdt_payment_args.into_vec().into_iter() {
            esdt_payments_vec.push(payment_args.into_esdt_token_payment());
        }
        let endpoint_name = match opt_function {
            OptionalArg::Some(data) => data,
            OptionalArg::None => ManagedBuffer::new(),
        };
        self.propose_action(Action::SendESDT {
            to,
            esdt_payments: esdt_payments_vec,
            endpoint_name,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCDeploy)]
    fn propose_sc_deploy(
        &self,
        amount: BigUint,
        code: ManagedBuffer,
        upgradeable: bool,
        payable: bool,
        readable: bool,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let code_metadata = get_code_metadata(upgradeable, payable, readable);
        self.propose_action(Action::SCDeploy {
            amount,
            code,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCDeployFromSource)]
    fn propose_sc_deploy_from_source(
        &self,
        amount: BigUint,
        source: ManagedAddress,
        upgradeable: bool,
        payable: bool,
        readable: bool,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let code_metadata = get_code_metadata(upgradeable, payable, readable);
        self.propose_action(Action::SCDeployFromSource {
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCUpgrade)]
    fn propose_sc_upgrade(
        &self,
        sc_address: ManagedAddress,
        amount: BigUint,
        code: ManagedBuffer,
        upgradeable: bool,
        payable: bool,
        readable: bool,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let code_metadata = get_code_metadata(upgradeable, payable, readable);
        self.propose_action(Action::SCUpgrade {
            sc_address,
            amount,
            code,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCUpgradeFromSource)]
    fn propose_sc_upgrade_from_source(
        &self,
        sc_address: ManagedAddress,
        amount: BigUint,
        source: ManagedAddress,
        upgradeable: bool,
        payable: bool,
        readable: bool,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let code_metadata = get_code_metadata(upgradeable, payable, readable);
        self.propose_action(Action::SCUpgradeFromSource {
            sc_address,
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }
}
