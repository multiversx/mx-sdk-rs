mod builtin_func_proxy;
mod delegation_manager_sc_proxy;
mod delegation_sc_proxy;
mod esdt_system_sc_proxy;
mod governance_sc_proxy;
pub(crate) mod token_properties;
mod token_properties_result;
mod validator_sc_proxy;

pub use builtin_func_proxy::*;
pub use delegation_manager_sc_proxy::*;
pub use delegation_sc_proxy::*;
pub use esdt_system_sc_proxy::{ESDTSystemSCProxy, ESDTSystemSCProxyMethods, IssueCall};
pub use governance_sc_proxy::{
    GovernanceSCProxy, GovernanceSCProxyMethods, governance_config_result::GovernanceConfigResult,
    proposal_view_result::ProposalViewResult,
};
pub use token_properties::*;
pub use token_properties_result::TokenPropertiesResult;
pub use validator_sc_proxy::*;

#[cfg(feature = "contract-call-legacy")]
mod legacy_system_sc_proxy;
#[cfg(feature = "contract-call-legacy")]
pub use legacy_system_sc_proxy::ESDTSystemSmartContractProxy;
