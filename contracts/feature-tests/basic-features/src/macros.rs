elrond_wasm::imports!();

use elrond_wasm::String;

/// Various macros provided by elrond-wasm.
#[elrond_wasm_derive::module]
pub trait Macros {
	#[view]
	fn only_owner(&self) -> SCResult<()> {
		only_owner!(self, "Caller must be owner");
		Ok(())
	}

	#[only_owner]
	#[endpoint]
	fn only_owner_endpoint(&self) -> SCResult<()> {
		Ok(())
	}

	#[view]
	fn require_equals(&self, a: u32, b: u32) -> SCResult<()> {
		require!(a == b, "a must equal b");
		Ok(())
	}

	#[view]
	fn return_sc_error(&self) -> SCResult<()> {
		sc_error!("return_sc_error")
	}

	#[view]
	fn result_ok(&self) -> SCResult<()> {
		SCResult::Ok(())
	}

	#[view]
	fn result_err_from_bytes_1(&self, e: BoxedBytes) -> SCResult<()> {
		SCResult::Err(e.into())?;
		unreachable!()
	}

	#[view]
	fn result_err_from_bytes_2<'a>(&self, e: &'a [u8]) -> SCResult<()> {
		SCResult::Err(e.into())
	}

	#[view]
	fn result_err_from_bytes_3(&self, e: Vec<u8>) -> SCResult<()> {
		SCResult::Err(e.into())
	}

	#[view]
	fn result_err_from_string(&self, e: String) -> SCResult<()> {
		SCResult::Err(e.into())
	}

	#[view]
	fn result_err_from_str<'a>(&self, e: &'a str) -> SCResult<()> {
		SCResult::Err(e.into())
	}

	#[endpoint]
	fn result_echo(&self, arg: Option<String>, test: bool) -> SCResult<String> {
		require!(test, "test argument is false");
		let unwrapped = SCResult::from_result(arg.ok_or("option argument is none"))?;
		Ok(unwrapped)
	}

	#[endpoint]
	fn result_echo_2(&self, arg: Option<String>) -> SCResult<String> {
		let unwrapped = arg.ok_or("option argument is none")?;
		Ok(unwrapped)
	}
}
