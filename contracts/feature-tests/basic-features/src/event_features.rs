elrond_wasm::imports!();

/// Tests event logs.
#[elrond_wasm_derive::module]
pub trait EventFeatures {
	#[endpoint(logEventA)]
	fn log_event_a(&self, data: &Self::BigUint) {
		self.event_a(data);
	}

	#[event("event_a")]
	fn event_a(&self, data: &Self::BigUint);

	#[endpoint(logEventB)]
	fn log_event_b(
		&self,
		arg1: &Self::BigUint,
		arg2: &Address,
		#[var_args] data: VarArgs<BoxedBytes>,
	) {
		self.event_b(arg1, arg2, data.as_slice());
	}

	#[event("event_b")]
	fn event_b(
		&self,
		#[indexed] arg1: &Self::BigUint,
		#[indexed] arg2: &Address,
		data: &[BoxedBytes],
	);

	// Legacy:

	#[endpoint(logLegacyEventA)]
	fn log_legacy_event_a(&self, data: &Self::BigUint) {
		self.legacy_event_a(data);
	}

	#[endpoint(logLegacyEventB)]
	fn log_legacy_event_b(&self, arg1: &Self::BigUint, arg2: &Address, data: &Self::BigUint) {
		self.legacy_event_b(arg1, arg2, data);
	}

	#[legacy_event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000a")]
	fn legacy_event_a(&self, data: &Self::BigUint);

	#[legacy_event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000b")]
	fn legacy_event_b(&self, arg1: &Self::BigUint, arg2: &Address, data: &Self::BigUint);
}
