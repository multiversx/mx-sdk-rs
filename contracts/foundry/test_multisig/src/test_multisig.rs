#![no_std]

use testapi;

multiversx_sc::imports!();

static OWNER : &[u8; 32]    = b"owner___________________________";
static ALICE : &[u8; 32]    = b"alice___________________________";
static BOB : &[u8; 32]      = b"bob_____________________________";
static CHARLIE : &[u8; 32]  = b"charlie_________________________";
static MULTISIG : &[u8; 32] = b"multisig________________________";

#[multiversx_sc::contract]
pub trait TestMultisigContract {

    #[init]
    fn init(&self, code_path: ManagedBuffer) {

        self.init_accounts();
        self.deploy(&code_path);

    }


    fn init_accounts(&self) {
        let owner = ManagedAddress::from(OWNER);
        testapi::create_account(&owner,                         0, &BigUint::from(0u64));
        testapi::create_account(&ManagedAddress::from(ALICE),   0, &BigUint::from(0u64));
        testapi::create_account(&ManagedAddress::from(BOB),     0, &BigUint::from(0u64));
        testapi::create_account(&ManagedAddress::from(CHARLIE), 0, &BigUint::from(0u64));
    
        let multisig = ManagedAddress::from(MULTISIG);
        testapi::register_new_address(&owner, 0, &multisig);

    }

    fn deploy(&self, code_path: &ManagedBuffer) {
        
        let mut init_args = ManagedArgBuffer::new();
        init_args.push_arg(2);                                    // quorum        = 2
        init_args.push_arg(ManagedAddress::from(ALICE));          // board members = alice, bob, charlie
        init_args.push_arg(ManagedAddress::from(BOB));
        init_args.push_arg(ManagedAddress::from(CHARLIE));
    
        let multisig = testapi::deploy_contract(
            &ManagedAddress::from(OWNER),
            5000000000000,
            &BigUint::zero(),
            code_path,
            &init_args,
        );

        testapi::assert( self.get_quorum(&multisig) == 2u32 );
        testapi::assert( self.get_num_board_members(&multisig) == 3u32 );

    }

    fn get_quorum(&self, multisig: &ManagedAddress) -> BigUint {
        let bs = testapi::get_storage(&multisig, &ManagedBuffer::from(b"quorum")); 
        BigUint::from(bs)
    }

    fn get_num_board_members(&self, multisig: &ManagedAddress) -> BigUint {
        let bs = testapi::get_storage(&multisig, &ManagedBuffer::from(b"num_board_members")); 
        BigUint::from(bs)
    }

    #[endpoint(test_change_quorum)]
    fn test_change_quorum(&self, value: BigUint) {
        let multisig = ManagedAddress::from(MULTISIG);
        let alice = ManagedAddress::from(ALICE);
        let bob = ManagedAddress::from(BOB);
        
        // make assumptions
        testapi::assume(value <= self.get_num_board_members(&multisig));
        

        self.change_quorum_propose(&multisig, &alice, &value);
        self.change_quorum_sign(&multisig, &bob);
        self.perform_action(&multisig, &alice);
    
        // check the final quorum
        testapi::assert(value == self.get_quorum(&multisig));
    }

    fn change_quorum_propose(&self, multisig: &ManagedAddress, proposer: &ManagedAddress, value: &BigUint) {
        let mut args = ManagedArgBuffer::new();
        args.push_arg(value);

        testapi::start_prank(&proposer);
        let _ = self.send_raw().direct_egld_execute(
            &multisig, 
            &BigUint::from(0u32), 
            5000000, 
            &ManagedBuffer::from(b"proposeChangeQuorum"),
            &args,
        );
        testapi::stop_prank();

    }

    fn change_quorum_sign(&self, multisig: &ManagedAddress, signer: &ManagedAddress) {
        let mut args = ManagedArgBuffer::new();
        args.push_arg(1u32);

        testapi::start_prank(signer);
        let _ = self.send_raw().direct_egld_execute(
            &multisig, 
            &BigUint::from(0u32), 
            5000000, 
            &ManagedBuffer::from(b"sign"),
            &args,
        );
        testapi::stop_prank();

    }

    fn perform_action(&self, multisig: &ManagedAddress, performer: &ManagedAddress) {
        let mut args = ManagedArgBuffer::new();
        args.push_arg(1u32);

        testapi::start_prank(performer);
        let _ = self.send_raw().direct_egld_execute(
            &multisig,
            &BigUint::from(0u32), 
            5000000, 
            &ManagedBuffer::from(b"performAction"),
            &args,
        );
        testapi::stop_prank();

    }
    
}
