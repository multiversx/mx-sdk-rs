elrond_wasm::imports!();

/// Tests event logs.
#[elrond_wasm::module]
pub trait EventFeatures {
    #[event("event_a")]
    fn event_a(&self, data: u32);

    #[endpoint(logEventA)]
    fn log_event_a(&self, data: u32) {
        self.event_a(data);
    }

    /// Logs `event_a` a repeated number of times.
    #[endpoint(logEventARepeat)]
    fn log_event_a_repeat(&self, num_logs: u32) {
        for i in 0..num_logs {
            self.event_a(i);
        }
    }

    #[event("event_b")]
    fn event_b(&self, #[indexed] arg1: &BigUint, #[indexed] arg2: &Address, data: &[BoxedBytes]);

    #[endpoint(logEventB)]
    fn log_event_b(
        &self,
        arg1: &BigUint,
        arg2: &Address,
        #[var_args] data: MultiValueVec<BoxedBytes>,
    ) {
        self.event_b(arg1, arg2, data.as_slice());
    }

    // Legacy:

    #[endpoint(logLegacyEventA)]
    fn log_legacy_event_a(&self, data: &BigUint) {
        self.legacy_event_a(data);
    }

    #[endpoint(logLegacyEventB)]
    fn log_legacy_event_b(&self, arg1: &BigUint, arg2: &Address, data: &BigUint) {
        self.legacy_event_b(arg1, arg2, data);
    }

    #[legacy_event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000a")]
    fn legacy_event_a(&self, data: &BigUint);

    #[legacy_event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000b")]
    fn legacy_event_b(&self, arg1: &BigUint, arg2: &Address, data: &BigUint);
}
