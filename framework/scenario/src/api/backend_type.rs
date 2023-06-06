/// Indicates what backend the `VMHooksApi` should use.
///
/// Should normally be an enum, but Rust doesn't yet support const generic enums.
/// Think of it as an enum in spirit and semantics.
/// I made it a char for easier debugging, if ever shows up there.
pub type VMHooksBackendType = char;

/// Indicates the `StaticApi`. i.e. an API that can support Managed Types in a static context, but nothing else.
pub const STATIC_MANAGED_TYPES: VMHooksBackendType = 's';

/// Indicates the defaut debugger API with its static stack.
pub const DEBUGGER_STACK: VMHooksBackendType = 'd';
