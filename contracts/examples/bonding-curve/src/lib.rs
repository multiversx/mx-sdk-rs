#![no_std]
#![allow(unused_attributes)]
#![feature(trait_alias)]
#![feature(destructuring_assignment)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod curves;
mod function_selector;
mod tokens;
mod utils;

use crate::tokens::{common_methods, fungible_token, non_fungible_token, semi_fungible_token};
use crate::utils::{events, storage};

#[elrond_wasm_derive::contract]
pub trait Contract:
	fungible_token::FungibleTokenModule
	+ non_fungible_token::NonFungibleTokenModule
	+ semi_fungible_token::SemiFungibleTokenModule
	+ storage::StorageModule
	+ events::EventsModule
	+ common_methods::CommonMethods
{
	#[init]
	fn init(&self) {}
}
