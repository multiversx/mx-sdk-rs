mod call_graph;
pub(crate) mod code_report;
pub(crate) mod endpoint_info;
pub(crate) mod extractor;
mod extractor_tests;
mod opcode_version;
mod opcode_whitelist;
pub(crate) mod report;
mod vm_hook_signature;
mod vm_hook_signature_list;

pub use call_graph::CallGraph;
pub use endpoint_info::{EndpointInfo, EndpointName, FunctionIndex, FunctionInfo};
pub use opcode_version::OpcodeVersion;
pub use vm_hook_signature::{FunctionType, VmHookSignature};
pub use vm_hook_signature_list::VM_HOOK_SIGNATURES;