#![no_std]

multiversx_sc::imports!();

use multiversx_sc_modules::pause;

#[multiversx_sc::contract]
pub trait CheckPauseContract: pause::PauseModule {
    #[init]
    fn init(&self) {}

    #[endpoint(checkPause)]
    fn check_pause(&self) -> bool {
        self.is_paused()
    }
}
