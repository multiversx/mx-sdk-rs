#![no_std]

mod multisig_proxy;

multiversx_sc::imports!();

static OWNER: &[u8; 32] = b"owner___________________________";
static ALICE: &[u8; 32] = b"alice___________________________";
static BOB: &[u8; 32] = b"bob_____________________________";
static CHARLIE: &[u8; 32] = b"charlie_________________________";
static MULTISIG: &[u8; 32] = b"multisig________________________";

#[multiversx_sc::contract]
pub trait TestMultisigContract {
    #[init]
    fn init(&self, code_path: ManagedBuffer) {
        self.init_accounts();
        self.deploy(&code_path);
    }

    fn init_accounts(&self) {
        let owner = ManagedAddress::from(OWNER);
        self.test_raw()
            .create_account(&owner, 0, &BigUint::from(0u64));
        self.test_raw()
            .create_account(&ManagedAddress::from(ALICE), 0, &BigUint::from(0u64));
        self.test_raw()
            .create_account(&ManagedAddress::from(BOB), 0, &BigUint::from(0u64));
        self.test_raw()
            .create_account(&ManagedAddress::from(CHARLIE), 0, &BigUint::from(0u64));

        let multisig = ManagedAddress::from(MULTISIG);
        self.test_raw().register_new_address(&owner, 0, &multisig);
    }

    fn deploy(&self, code_path: &ManagedBuffer) {
        let mut board = MultiValueEncoded::new();
        board.push(ManagedAddress::from(ALICE)); // board members = alice, bob, charlie
        board.push(ManagedAddress::from(BOB));
        board.push(ManagedAddress::from(CHARLIE));

        let multisig = self
            .tx()
            .from(ManagedAddress::from(OWNER))
            .typed(multisig_proxy::MultisigProxy)
            .init(2usize, board)
            .code_path(code_path)
            .gas(5000000000000)
            .returns(ReturnsNewManagedAddress)
            .test_deploy();

        self.test_raw().assert(self.get_quorum(&multisig) == 2usize);
        self.test_raw()
            .assert(self.get_num_board_members(&multisig) == 3usize);
    }

    fn get_quorum(&self, multisig: &ManagedAddress) -> usize {
        self.storage_raw().read_from_address(multisig, "quorum")
    }

    fn get_num_board_members(&self, multisig: &ManagedAddress) -> usize {
        self.storage_raw()
            .read_from_address(multisig, "num_board_members")
    }

    #[endpoint(test_change_quorum)]
    fn test_change_quorum(&self, value: usize) {
        let multisig = ManagedAddress::from(MULTISIG);
        let alice = ManagedAddress::from(ALICE);
        let bob = ManagedAddress::from(BOB);

        // make assumptions
        self.test_raw()
            .assume(value <= self.get_num_board_members(&multisig));

        self.change_quorum_propose(&multisig, &alice, value);
        self.change_quorum_sign(&multisig, &bob);
        self.perform_action(&multisig, &alice);

        // check the final quorum
        self.test_raw().assert(value == self.get_quorum(&multisig));
    }

    fn change_quorum_propose(
        &self,
        multisig: &ManagedAddress,
        proposer: &ManagedAddress,
        value: usize,
    ) {
        self.tx()
            .from(proposer)
            .to(multisig)
            .typed(multisig_proxy::MultisigProxy)
            .propose_change_quorum(value)
            .gas(5000000)
            .test_call();
    }

    fn change_quorum_sign(&self, multisig: &ManagedAddress, signer: &ManagedAddress) {
        self.tx()
            .from(signer)
            .to(multisig)
            .typed(multisig_proxy::MultisigProxy)
            .sign(1usize)
            .gas(5000000)
            .test_call();
    }

    fn perform_action(&self, multisig: &ManagedAddress, performer: &ManagedAddress) {
        self.tx()
            .from(performer)
            .to(multisig)
            .typed(multisig_proxy::MultisigProxy)
            .perform_action_endpoint(1usize)
            .gas(5000000)
            .test_call();
    }
}
