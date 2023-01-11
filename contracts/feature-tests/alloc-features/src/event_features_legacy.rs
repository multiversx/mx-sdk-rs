multiversx_sc::imports!();

/// Legacy event logs.
///
/// They are the only ones that still use the old write logs VM endpoint.
#[multiversx_sc::module]
pub trait EventFeaturesLegacy {
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
