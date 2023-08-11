// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           17
// Async Callback:                       1
// Total number of exported functions:  19

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    cryptozombies
    (
        init => init
        set_crypto_kitties_sc_address => set_crypto_kitties_sc_address
        generate_random_dna => generate_random_dna
        create_random_zombie => create_random_zombie
        is_ready => is_ready
        feed_on_kitty => feed_on_kitty
        dna_digits => dna_digits
        zombies_count => zombies_count
        zombies => zombies
        zombie_owner => zombie_owner
        crypto_kitties_sc_address => crypto_kitties_sc_address
        cooldown_time => cooldown_time
        owned_zombies => owned_zombies
        level_up => level_up
        withdraw => withdraw
        change_name => change_name
        change_dna => change_dna
        attack => attack
    )
}

multiversx_sc_wasm_adapter::async_callback! { cryptozombies }
