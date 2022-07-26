elrond_wasm::imports!();

/// Standard smart contract module that, when added to a smart contract, offers pausability.
///
/// It provides a flag that contracts can use to check if owner decided to pause the entire contract.
/// Use the features module for more granular on/off switches.
///
/// It offers:
/// * an endpoint where the owner can pause/unpause contract
/// * a method to check if contract is paused or not
///
#[elrond_wasm::module]
pub trait PauseModule {
    #[view(isPaused)]
    #[storage_get("pause_module:paused")]
    fn is_paused(&self) -> bool;

    #[inline]
    fn not_paused(&self) -> bool {
        !self.is_paused()
    }

    #[storage_set("pause_module:paused")]
    fn set_paused(&self, paused: bool);

    #[only_owner]
    #[endpoint(pause)]
    fn pause_endpoint(&self) {
        self.set_paused(true);
        // TODO: event
    }

    #[only_owner]
    #[endpoint(unpause)]
    fn unpause_endpoint(&self) {
        self.set_paused(false);
        // TODO: event
    }

    fn require_paused(&self) {
        require!(self.is_paused(), "Contract is not paused");
    }

    fn require_not_paused(&self) {
        require!(self.not_paused(), "Contract is paused");
    }
}
