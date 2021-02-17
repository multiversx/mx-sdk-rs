#![allow(unused_imports)]

use crate::api::{BigUintApi, ErrorApi, SendApi, ESDT_TRANSFER_STRING};
use crate::hex_call_data::HexCallDataSerializer;
use crate::io::AsyncCallArg;
use crate::io::EndpointResult;
use crate::types::{Address, SCError};
use crate::{
	abi::{OutputAbi, TypeAbi, TypeDescriptionContainer},
	TokenIdentifier,
};
use alloc::string::String;
use alloc::vec::Vec;

pub struct CallbackCall {
	pub closure_data: HexCallDataSerializer,
}

impl CallbackCall {
	pub fn new(callback_name: &[u8]) -> Self {
		CallbackCall {
			closure_data: HexCallDataSerializer::new(callback_name),
		}
	}

	pub fn from_raw(closure_data: HexCallDataSerializer) -> Self {
		CallbackCall { closure_data }
	}

	pub fn push_callback_argument_raw_bytes(&mut self, bytes: &[u8]) {
		self.closure_data.push_argument_bytes(bytes);
	}
}
