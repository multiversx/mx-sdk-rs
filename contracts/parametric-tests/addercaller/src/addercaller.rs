// A smart contract to test transfer & execute functions
// Initialize the contract with the address of the adder
// The endpoints `call_adder` and `call_adder_esdt` accepts
// tokens in EGLD and ESDT and performs transfer & execute
// to the adder's `add` endpoint.

#![no_std]

multiversx_sc::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait AdderCaller {
    #[storage_mapper("dest")]
    fn dest(&self) -> SingleValueMapper<ManagedAddress>;

    #[init]
    fn init(&self, dest: &ManagedAddress) {
        self.dest().set(dest);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn call_adder(&self, value: BigUint) -> ManagedBuffer {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(value);

        let result = self.send_raw().direct_egld_execute(
            &self.dest().get(),
            &BigUint::from(30u32),
            5000000,
            &ManagedBuffer::from(b"add"),
            &arg_buffer,
        );

        match result {
            Result::Err(e) => sc_panic!(e),
            Result::Ok(_) => ManagedBuffer::from("added"),
        }
    }

    #[endpoint]
    #[payable("MYESDT")]
    fn call_adder_esdt(&self, value: BigUint) -> ManagedBuffer {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(value);

        let result = self.send_raw().transfer_esdt_execute(
            &self.dest().get(),
            &TokenIdentifier::from_esdt_bytes(b"MYESDT"),
            &BigUint::from(20u32),
            5000000,
            &ManagedBuffer::from(b"add"),
            &arg_buffer,
        );

        match result {
            Result::Err(e) => sc_panic!(e),
            Result::Ok(_) => ManagedBuffer::from("added-esdt"),
        }
    }
}

//
