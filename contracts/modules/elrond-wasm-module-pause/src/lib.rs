#![no_std]
#![allow(clippy::string_lit_as_bytes)]

#[macro_use]
extern crate elrond_wasm;

elrond_wasm::imports!();

/// The module deals with temporarily pausing contract operations.
/// It provides a flag that contracts can use to check if owner decided to pause the entire contract.
/// Use the features module for more granular on/off switches.
#[elrond_wasm_derive::module(PauseModuleImpl)]
pub trait PauseModule {
	#[view(isPaused)]
	#[storage_get("pause_module:paused")]
	fn is_paused(&self) -> bool;

	fn not_paused(&self) -> bool {
		!self.is_paused()
	}

	#[storage_set("pause_module:paused")]
	fn set_paused(&self, paused: bool);

	#[endpoint(pause)]
	fn pause_endpoint(&self) -> SCResult<()> {
		require!(
			self.get_caller() == self.get_owner_address(),
			"only owner allowed to pause contract"
		);

		self.set_paused(true);
		// TODO: event
		Ok(())
	}

	#[endpoint(unpause)]
	fn unpause_endpoint(&self) -> SCResult<()> {
		require!(
			self.get_caller() == self.get_owner_address(),
			"only owner allowed to unpause contract"
		);

		self.set_paused(false);
		// TODO: event
		Ok(())
	}
}
