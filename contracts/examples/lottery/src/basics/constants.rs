use multiversx_sc::proxy_imports::DurationMillis;

pub const PERCENTAGE_TOTAL: u32 = 100;
pub const THIRTY_DAYS_IN_MILLISECONDS: DurationMillis =
    DurationMillis::new(60 * 60 * 24 * 30 * 1000);
pub const MAX_TICKETS: usize = 800;
pub const MAX_OPERATIONS: usize = 50;
