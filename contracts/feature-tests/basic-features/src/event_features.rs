multiversx_sc::imports!();

/// Tests event logs.
#[multiversx_sc::module]
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
    fn event_b(
        &self,
        #[indexed] arg1: &BaseBigUint,
        #[indexed] arg2: &ManagedAddress,
        data: ManagedVec<ManagedBuffer>,
    );

    #[endpoint(logEventB)]
    fn log_event_b(
        &self,
        arg1: &BaseBigUint,
        arg2: &ManagedAddress,
        data: MultiValueManagedVec<ManagedBuffer>,
    ) {
        self.event_b(arg1, arg2, data.into_vec());
    }
}
