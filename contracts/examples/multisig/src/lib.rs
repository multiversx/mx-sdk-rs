#![no_std]

mod action;
mod user_role;

use action::{Action, ActionFullInfo, PerformActionResult};
use user_role::UserRole;

elrond_wasm::imports!();

#[cfg(feature = "elrond-wasm-module-users-default")]
use elrond_wasm_module_users_default::*;
#[cfg(feature = "elrond-wasm-module-users-wasm")]
use elrond_wasm_module_users_wasm::*;

/// Multi-signature smart contract implementation.
/// Acts like a wallet that needs multiple signers for any action performed.
/// See the readme file for more detailed documentation.
#[elrond_wasm_derive::contract(MultisigImpl)]
pub trait Multisig {
	#[module(UsersModuleImpl)]
	fn users_module(&self) -> UsersModuleImpl<T, BigInt, BigUint>;

	/// Minimum number of signatures needed to perform any action.
	#[view(getQuorum)]
	#[storage_get("quorum")]
	fn get_quorum(&self) -> usize;

	#[storage_set("quorum")]
	fn set_quorum(&self, quorum: usize);

	#[storage_get("user_role")]
	fn get_user_id_to_role(&self, user_id: usize) -> UserRole;

	#[storage_set("user_role")]
	fn set_user_id_to_role(&self, user_id: usize, user_role: UserRole);

	/// Denormalized board member count.
	/// It is kept in sync with the user list by the contract.
	#[view(getNumBoardMembers)]
	#[storage_get("num_board_members")]
	fn get_num_board_members(&self) -> usize;

	#[storage_set("num_board_members")]
	fn set_num_board_members(&self, num_board_members: usize);

	/// Denormalized proposer count.
	/// It is kept in sync with the user list by the contract.
	#[view(getNumProposers)]
	#[storage_get("num_proposers")]
	fn get_num_proposers(&self) -> usize;

	#[storage_set("num_proposers")]
	fn set_num_proposers(&self, num_proposers: usize);

	/// The index of the last proposed action.
	/// 0 means that no action was ever proposed yet.
	#[view(getActionLastIndex)]
	#[storage_get("action_last_index")]
	fn get_action_last_index(&self) -> usize;

	#[storage_set("action_last_index")]
	fn set_action_last_index(&self, action_last_index: usize);

	/// Serialized action data of an action with index.
	#[view(getActionData)]
	#[storage_get("action_data")]
	fn get_action_data(&self, action_id: usize) -> Action<BigUint>;

	#[storage_set("action_data")]
	fn set_action_data(&self, action_id: usize, action_data: &Action<BigUint>);

	/// Equivalent to `!self.get_action_data(action_id).is_pending()`,
	/// but more efficient because it does not need to deserialize the data.
	#[storage_is_empty("action_data")]
	fn is_empty_action_data(&self, action_id: usize) -> bool;

	#[storage_get("action_signer_ids")]
	fn get_action_signer_ids(&self, action_id: usize) -> Vec<usize>;

	#[storage_set("action_signer_ids")]
	fn set_action_signer_ids(&self, action_id: usize, action_signer_ids: &[usize]);

	#[init]
	fn init(&self, quorum: usize, #[var_args] board: VarArgs<Address>) -> SCResult<()> {
		require!(
			!board.is_empty(),
			"board cannot be empty on init, no-one would be able to propose"
		);
		require!(quorum <= board.len(), "quorum cannot exceed board size");
		self.set_quorum(quorum);

		for (i, address) in board.iter().enumerate() {
			let user_id = i + 1;
			require!(
				self.users_module().get_user_id(&address) == 0,
				"duplicate board member"
			);
			self.users_module().set_user_id(&address, user_id);
			self.users_module().set_user_address(user_id, &address);
			self.set_user_id_to_role(user_id, UserRole::BoardMember);
		}
		self.users_module().set_num_users(board.len());
		self.set_num_board_members(board.len());

		Ok(())
	}

	/// Allows the contract to receive funds even if it is marked as unpayable in the protocol.
	#[payable("*")]
	#[endpoint]
	fn deposit(&self) {}

	fn propose_action(&self, action: Action<BigUint>) -> SCResult<usize> {
		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(
			caller_role.can_propose(),
			"only board members and proposers can propose"
		);

		let action_id = self.get_action_last_index() + 1;
		self.set_action_last_index(action_id);
		self.set_action_data(action_id, &action);
		if caller_role.can_sign() {
			// also sign
			// since the action is newly created, the caller can be the only signer
			self.set_action_signer_ids(action_id, &[caller_id][..]);
		}

		Ok(action_id)
	}

	/// Iterates through all actions and retrieves those that are still pending.
	/// Serialized full action data:
	/// - the action id
	/// - the serialized action data
	/// - (number of signers followed by) list of signer addresses.
	#[view(getPendingActionFullInfo)]
	fn get_pending_action_full_info(&self) -> MultiResultVec<ActionFullInfo<BigUint>> {
		let mut result = Vec::new();
		let action_last_index = self.get_action_last_index();
		for action_id in 1..=action_last_index {
			let action_data = self.get_action_data(action_id);
			if action_data.is_pending() {
				result.push(ActionFullInfo {
					action_id,
					action_data,
					signers: self.get_action_signers(action_id),
				});
			}
		}
		result.into()
	}

	/// Initiates board member addition process.
	/// Can also be used to promote a proposer to board member.
	#[endpoint(proposeAddBoardMember)]
	fn propose_add_board_member(&self, board_member_address: Address) -> SCResult<usize> {
		self.propose_action(Action::AddBoardMember(board_member_address))
	}

	/// Initiates proposer addition process..
	/// Can also be used to demote a board member to proposer.
	#[endpoint(proposeAddProposer)]
	fn propose_add_proposer(&self, proposer_address: Address) -> SCResult<usize> {
		self.propose_action(Action::AddProposer(proposer_address))
	}

	/// Removes user regardless of whether it is a board member or proposer.
	#[endpoint(proposeRemoveUser)]
	fn propose_remove_user(&self, user_address: Address) -> SCResult<usize> {
		self.propose_action(Action::RemoveUser(user_address))
	}

	#[endpoint(proposeChangeQuorum)]
	fn propose_change_quorum(&self, new_quorum: usize) -> SCResult<usize> {
		self.propose_action(Action::ChangeQuorum(new_quorum))
	}

	#[endpoint(proposeSendEgld)]
	fn propose_send_egld(
		&self,
		to: Address,
		amount: BigUint,
		#[var_args] opt_data: OptionalArg<BoxedBytes>,
	) -> SCResult<usize> {
		let data = match opt_data {
			OptionalArg::Some(data) => data,
			OptionalArg::None => BoxedBytes::empty(),
		};
		self.propose_action(Action::SendEgld { to, amount, data })
	}

	#[endpoint(proposeSCDeploy)]
	fn propose_sc_deploy(
		&self,
		amount: BigUint,
		code: BoxedBytes,
		upgradeable: bool,
		payable: bool,
		readable: bool,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) -> SCResult<usize> {
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
		self.propose_action(Action::SCDeploy {
			amount,
			code,
			code_metadata,
			arguments: arguments.into_vec(),
		})
	}

	/// To be used not only for smart contract calls,
	/// but also for ESDT calls or any protocol built-in function.
	#[endpoint(proposeSCCall)]
	fn propose_sc_call(
		&self,
		to: Address,
		egld_payment: BigUint,
		endpoint_name: BoxedBytes,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) -> SCResult<usize> {
		self.propose_action(Action::SCCall {
			to,
			egld_payment,
			endpoint_name,
			arguments: arguments.into_vec(),
		})
	}

	/// Returns `true` (`1`) if the user has signed the action.
	/// Does not check whether or not the user is still a board member and the signature valid.
	#[view]
	fn signed(&self, user: Address, action_id: usize) -> bool {
		let user_id = self.users_module().get_user_id(&user);
		if user_id == 0 {
			false
		} else {
			let signer_ids = self.get_action_signer_ids(action_id);
			signer_ids.contains(&user_id)
		}
	}

	/// Indicates user rights.
	/// `0` = no rights,
	/// `1` = can propose, but not sign,
	/// `2` = can propose and sign.
	#[view(userRole)]
	fn user_role(&self, user: Address) -> UserRole {
		let user_id = self.users_module().get_user_id(&user);
		if user_id == 0 {
			UserRole::None
		} else {
			self.get_user_id_to_role(user_id)
		}
	}

	/// Lists all users that can sign actions.
	#[view(getAllBoardMembers)]
	fn get_all_board_members(&self) -> MultiResultVec<Address> {
		self.get_all_users_with_role(UserRole::BoardMember)
	}

	/// Lists all proposers that are not board members.
	#[view(getAllProposers)]
	fn get_all_proposers(&self) -> MultiResultVec<Address> {
		self.get_all_users_with_role(UserRole::Proposer)
	}

	fn get_all_users_with_role(&self, role: UserRole) -> MultiResultVec<Address> {
		let mut result = Vec::new();
		let num_users = self.users_module().get_num_users();
		for user_id in 1..=num_users {
			if self.get_user_id_to_role(user_id) == role {
				result.push(self.users_module().get_user_address(user_id));
			}
		}
		result.into()
	}

	/// Used by board members to sign actions.
	#[endpoint]
	fn sign(&self, action_id: usize) -> SCResult<()> {
		require!(
			!self.is_empty_action_data(action_id),
			"action does not exist"
		);

		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(caller_role.can_sign(), "only board members can sign");

		let mut signer_ids = self.get_action_signer_ids(action_id);
		if !signer_ids.contains(&caller_id) {
			signer_ids.push(caller_id);
			self.set_action_signer_ids(action_id, signer_ids.as_slice());
		}

		Ok(())
	}

	/// Board members can withdraw their signatures if they no longer desire for the action to be executed.
	/// Actions that are left with no valid signatures can be then deleted to free up storage.
	#[endpoint]
	fn unsign(&self, action_id: usize) -> SCResult<()> {
		require!(
			!self.is_empty_action_data(action_id),
			"action does not exist"
		);

		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(caller_role.can_sign(), "only board members can un-sign");

		let mut signer_ids = self.get_action_signer_ids(action_id);
		if let Some(signer_pos) = signer_ids
			.iter()
			.position(|signer_id| *signer_id == caller_id)
		{
			// since we don't care about the order,
			// it is ok to call swap_remove, which is O(1)
			signer_ids.swap_remove(signer_pos);
			self.set_action_signer_ids(action_id, signer_ids.as_slice());
		}

		Ok(())
	}

	/// Can be used to:
	/// - create new user (board member / proposer)
	/// - remove user (board member / proposer)
	/// - reactivate removed user
	/// - convert between board member and proposer
	/// Will keep the board size and proposer count in sync.
	fn change_user_role(&self, user_address: Address, new_role: UserRole) {
		let user_id = self.users_module().get_or_create_user(&user_address);
		let old_role = if user_id == 0 {
			UserRole::None
		} else {
			self.get_user_id_to_role(user_id)
		};
		self.set_user_id_to_role(user_id, new_role);

		// update board size
		#[allow(clippy::collapsible_if)]
		if old_role == UserRole::BoardMember {
			if new_role != UserRole::BoardMember {
				self.set_num_board_members(self.get_num_board_members() - 1);
			}
		} else {
			if new_role == UserRole::BoardMember {
				self.set_num_board_members(self.get_num_board_members() + 1);
			}
		}

		// update num_proposers
		#[allow(clippy::collapsible_if)]
		if old_role == UserRole::Proposer {
			if new_role != UserRole::Proposer {
				self.set_num_proposers(self.get_num_proposers() - 1);
			}
		} else {
			if new_role == UserRole::Proposer {
				self.set_num_proposers(self.get_num_proposers() + 1);
			}
		}
	}

	/// Gets addresses of all users who signed an action.
	/// Does not check if those users are still board members or not,
	/// so the result may contain invalid signers.
	#[view(getActionSigners)]
	fn get_action_signers(&self, action_id: usize) -> Vec<Address> {
		self.get_action_signer_ids(action_id)
			.iter()
			.map(|signer_id| self.users_module().get_user_address(*signer_id))
			.collect()
	}

	/// Gets addresses of all users who signed an action and are still board members.
	/// All these signatures are currently valid.
	#[view(getActionSignerCount)]
	fn get_action_signer_count(&self, action_id: usize) -> usize {
		self.get_action_signer_ids(action_id).len()
	}

	/// It is possible for board members to lose their role.
	/// They are not automatically removed from all actions when doing so,
	/// therefore the contract needs to re-check every time when actions are performed.
	/// This function is used to validate the signers before performing an action.
	/// It also makes it easy to check before performing an action.
	#[view(getActionValidSignerCount)]
	fn get_action_valid_signer_count(&self, action_id: usize) -> usize {
		let signer_ids = self.get_action_signer_ids(action_id);
		signer_ids
			.iter()
			.filter(|signer_id| {
				let signer_role = self.get_user_id_to_role(**signer_id);
				signer_role.can_sign()
			})
			.count()
	}

	/// Returns `true` (`1`) if `getActionValidSignerCount >= getQuorum`.
	#[view(quorumReached)]
	fn quorum_reached(&self, action_id: usize) -> bool {
		let quorum = self.get_quorum();
		let valid_signers_count = self.get_action_valid_signer_count(action_id);
		valid_signers_count >= quorum
	}

	/// Proposers and board members use this to launch signed actions.
	#[endpoint(performAction)]
	fn perform_action_endpoint(&self, action_id: usize) -> SCResult<PerformActionResult<BigUint>> {
		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(
			caller_role.can_perform_action(),
			"only board members and proposers can perform actions"
		);
		require!(
			self.quorum_reached(action_id),
			"quorum has not been reached"
		);

		self.perform_action(action_id)
	}

	fn perform_action(&self, action_id: usize) -> SCResult<PerformActionResult<BigUint>> {
		let action = self.get_action_data(action_id);

		// clean up storage
		// happens before actual execution, because the match provides the return on each branch
		// syntax aside, the async_call_raw kills contract execution so cleanup cannot happen afterwards
		self.clear_action(action_id);

		match action {
			Action::Nothing => Ok(PerformActionResult::Nothing),
			Action::AddBoardMember(board_member_address) => {
				self.change_user_role(board_member_address, UserRole::BoardMember);
				Ok(PerformActionResult::Nothing)
			},
			Action::AddProposer(proposer_address) => {
				self.change_user_role(proposer_address, UserRole::Proposer);

				// validation required for the scenario when a board member becomes a proposer
				require!(
					self.get_quorum() <= self.get_num_board_members(),
					"quorum cannot exceed board size"
				);
				Ok(PerformActionResult::Nothing)
			},
			Action::RemoveUser(user_address) => {
				self.change_user_role(user_address, UserRole::None);
				let num_board_members = self.get_num_board_members();
				let num_proposers = self.get_num_proposers();
				require!(
					num_board_members + num_proposers > 0,
					"cannot remove all board members and proposers"
				);
				require!(
					self.get_quorum() <= num_board_members,
					"quorum cannot exceed board size"
				);
				Ok(PerformActionResult::Nothing)
			},
			Action::ChangeQuorum(new_quorum) => {
				require!(
					new_quorum <= self.get_num_board_members(),
					"quorum cannot exceed board size"
				);
				self.set_quorum(new_quorum);
				Ok(PerformActionResult::Nothing)
			},
			Action::SendEgld { to, amount, data } => {
				Ok(PerformActionResult::SendEgld(SendEgld { to, amount, data }))
			},
			Action::SCDeploy {
				amount,
				code,
				code_metadata,
				arguments,
			} => {
				let gas_left = self.get_gas_left();
				let mut arg_buffer = ArgBuffer::new();
				for arg in arguments {
					arg_buffer.push_argument_bytes(arg.as_slice());
				}
				let new_address = self.send().deploy_contract(
					gas_left,
					&amount,
					&code,
					code_metadata,
					&arg_buffer,
				);
				Ok(PerformActionResult::DeployResult(new_address))
			},
			Action::SCCall {
				to,
				egld_payment,
				endpoint_name,
				arguments,
			} => {
				let mut contract_call_raw =
					ContractCall::new(to, TokenIdentifier::egld(), egld_payment, endpoint_name);
				for arg in arguments {
					contract_call_raw.push_argument_raw_bytes(arg.as_slice());
				}
				Ok(PerformActionResult::AsyncCall(
					contract_call_raw.async_call(),
				))
			},
		}
	}

	fn clear_action(&self, action_id: usize) {
		self.set_action_data(action_id, &Action::Nothing);
		self.set_action_signer_ids(action_id, &[][..]);
	}

	/// Clears storage pertaining to an action that is no longer supposed to be executed.
	/// Any signatures that the action received must first be removed, via `unsign`.
	/// Otherwise this endpoint would be prone to abuse.
	#[endpoint(discardAction)]
	fn discard_action(&self, action_id: usize) -> SCResult<()> {
		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(
			caller_role.can_discard_action(),
			"only board members and proposers can discard actions"
		);
		require!(
			self.get_action_valid_signer_count(action_id) == 0,
			"cannot discard action with valid signatures"
		);

		self.clear_action(action_id);
		Ok(())
	}
}
