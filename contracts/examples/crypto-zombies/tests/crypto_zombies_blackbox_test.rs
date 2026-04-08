use multiversx_sc_scenario::imports::*;

use crypto_zombies::proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const USER1_ADDRESS: TestAddress = TestAddress::new("user1");
const USER2_ADDRESS: TestAddress = TestAddress::new("user2");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("crypto-zombies");
const CODE_PATH: MxscPath = MxscPath::new("output/crypto-zombies.mxsc.json");

// 0.001 EGLD – matches the fee set in init()
const LEVEL_UP_FEE: u64 = 1_000_000_000_000_000;
// Default cooldown set in init(): 86 400 ms
const COOLDOWN_TIME: DurationMillis = DurationMillis::new(86_400);

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crypto-zombies");
    blockchain.register_contract(CODE_PATH, crypto_zombies::ContractBuilder);
    blockchain
}

// ---------------------------------------------------------------------------
// Test-state helper
// ---------------------------------------------------------------------------

struct CryptoZombiesState {
    world: ScenarioWorld,
}

impl CryptoZombiesState {
    fn new() -> Self {
        let mut world = world();
        world
            .account(OWNER_ADDRESS)
            .nonce(1)
            .balance(LEVEL_UP_FEE * 10);
        world
            .account(USER1_ADDRESS)
            .nonce(0)
            .balance(LEVEL_UP_FEE * 10);
        world
            .account(USER2_ADDRESS)
            .nonce(0)
            .balance(LEVEL_UP_FEE * 10);
        Self { world }
    }

    fn deploy(&mut self) -> &mut Self {
        self.world
            .tx()
            .id("deploy")
            .from(OWNER_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .init()
            .code(CODE_PATH)
            .new_address(SC_ADDRESS)
            .run();
        self
    }

    fn create_zombie(&mut self, from: TestAddress, name: &str) -> &mut Self {
        self.world
            .tx()
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .create_random_zombie(ManagedBuffer::from(name))
            .run();
        self
    }

    fn create_zombie_expect_err(&mut self, from: TestAddress, name: &str, err: &str) -> &mut Self {
        self.world
            .tx()
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .create_random_zombie(ManagedBuffer::from(name))
            .returns(ExpectError(4, err))
            .run();
        self
    }

    fn level_up_zombie(&mut self, from: TestAddress, zombie_id: usize) -> &mut Self {
        self.world
            .tx()
            .id("level-up")
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .level_up(zombie_id)
            .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, LEVEL_UP_FEE).unwrap())
            .run();
        self
    }

    fn level_up_zombie_expect_err(
        &mut self,
        from: TestAddress,
        zombie_id: usize,
        payment: u64,
        err: &str,
    ) -> &mut Self {
        self.world
            .tx()
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .level_up(zombie_id)
            .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, payment).unwrap())
            .returns(ExpectError(4, err))
            .run();
        self
    }

    fn change_name(&mut self, from: TestAddress, zombie_id: usize, name: &str) -> &mut Self {
        self.world
            .tx()
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .change_name(zombie_id, ManagedBuffer::from(name))
            .run();
        self
    }

    fn change_name_expect_err(
        &mut self,
        from: TestAddress,
        zombie_id: usize,
        name: &str,
        err: &str,
    ) -> &mut Self {
        self.world
            .tx()
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .change_name(zombie_id, ManagedBuffer::from(name))
            .returns(ExpectError(4, err))
            .run();
        self
    }

    fn change_dna_expect_err(
        &mut self,
        from: TestAddress,
        zombie_id: usize,
        dna: u64,
        err: &str,
    ) -> &mut Self {
        self.world
            .tx()
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .change_dna(zombie_id, dna)
            .returns(ExpectError(4, err))
            .run();
        self
    }

    fn attack(&mut self, from: TestAddress, zombie_id: usize, target_id: usize) -> &mut Self {
        self.world
            .tx()
            .id("attack")
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .attack(zombie_id, target_id)
            .run();
        self
    }

    fn attack_expect_err(
        &mut self,
        from: TestAddress,
        zombie_id: usize,
        target_id: usize,
        err: &str,
    ) -> &mut Self {
        self.world
            .tx()
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .attack(zombie_id, target_id)
            .returns(ExpectError(4, err))
            .run();
        self
    }

    fn withdraw(&mut self, from: TestAddress) -> &mut Self {
        self.world
            .tx()
            .id("withdraw")
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .withdraw()
            .run();
        self
    }

    fn withdraw_expect_err(&mut self, from: TestAddress, err: &str) -> &mut Self {
        self.world
            .tx()
            .from(from)
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .withdraw()
            .returns(ExpectError(4, err))
            .run();
        self
    }

    fn query_zombie_last_index(&mut self) -> usize {
        self.world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .zombie_last_index()
            .returns(ReturnsResultUnmanaged)
            .run()
    }

    fn query_zombie(&mut self, id: usize) -> proxy::Zombie<StaticApi> {
        self.world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .zombies(id)
            .returns(ReturnsResult)
            .run()
    }

    fn query_is_ready(&mut self, zombie_id: usize) -> bool {
        self.world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .is_ready(zombie_id)
            .returns(ReturnsResultUnmanaged)
            .run()
    }

    fn query_dna_digits(&mut self) -> u8 {
        self.world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .dna_digits()
            .returns(ReturnsResultUnmanaged)
            .run()
    }

    fn query_cooldown_time(&mut self) -> DurationMillis {
        self.world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy::CryptoZombiesProxy)
            .cooldown_time()
            .returns(ReturnsResultUnmanaged)
            .run()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_deploy_init_values() {
    let mut state = CryptoZombiesState::new();
    state.deploy();

    let dna_digits = state.query_dna_digits();
    assert_eq!(dna_digits, 16u8);

    let cooldown_time = state.query_cooldown_time();
    assert_eq!(cooldown_time, COOLDOWN_TIME);

    // No zombies after deploy
    let index = state.query_zombie_last_index();
    assert_eq!(index, 0usize);
}

#[test]
fn test_create_random_zombie() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");

    // One zombie has been registered
    let index = state.query_zombie_last_index();
    assert_eq!(index, 1usize);

    // Check zombie has the expected defaults
    let zombie = state.query_zombie(0);
    assert_eq!(zombie.level, 1u16);
    assert_eq!(zombie.win_count, 0usize);
    assert_eq!(zombie.loss_count, 0usize);
    assert_eq!(zombie.name, ManagedBuffer::from("Alpha"));
}

#[test]
fn test_create_zombie_already_owns_one() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");

    state.create_zombie_expect_err(USER1_ADDRESS, "Beta", "You already own a zombie");
}

#[test]
fn test_zombie_is_ready_at_creation() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");

    // Zombie's ready_time == current block time, so is_ready should be true
    let ready = state.query_is_ready(0);
    assert!(ready, "Newly created zombie should be immediately ready");
}

#[test]
fn test_zombie_not_ready_after_cooldown_trigger() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state
        .world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(0));

    state.create_zombie(USER1_ADDRESS, "Alpha");
    state.create_zombie(USER2_ADDRESS, "Beta");

    // Attack triggers cooldown on zombie 0 regardless of win or loss
    state.attack(USER1_ADDRESS, 0, 1);

    // Zombie 0 now has ready_time = COOLDOWN_TIME in the future; still at t=0 so not ready
    let ready = state.query_is_ready(0);
    assert!(
        !ready,
        "Zombie should NOT be ready right after cooldown trigger"
    );
}

#[test]
fn test_zombie_ready_after_cooldown_expires() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state
        .world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(0));

    state.create_zombie(USER1_ADDRESS, "Alpha");
    state.create_zombie(USER2_ADDRESS, "Beta");

    state.attack(USER1_ADDRESS, 0, 1);

    // Advance time past the cooldown
    state
        .world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(1u64 + COOLDOWN_TIME.as_u64_millis()));

    let ready = state.query_is_ready(0);
    assert!(
        ready,
        "Zombie should be ready after cooldown period expires"
    );
}

#[test]
fn test_level_up_correct_payment() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");

    let before = state.query_zombie(0);
    assert_eq!(before.level, 1u16);

    state.level_up_zombie(USER1_ADDRESS, 0);

    let after = state.query_zombie(0);
    assert_eq!(after.level, 2u16);
}

#[test]
fn test_level_up_wrong_payment_fails() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");

    state.level_up_zombie_expect_err(
        USER1_ADDRESS,
        0,
        LEVEL_UP_FEE - 1,
        "Payment must be equal to the level up fee",
    );
}

#[test]
fn test_change_name_requires_level_2() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");

    // Level 1 – should fail
    state.change_name_expect_err(USER1_ADDRESS, 0, "NewName", "Zombie is too low level");
}

#[test]
fn test_change_name_at_level_2() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");

    state.level_up_zombie(USER1_ADDRESS, 0);

    // Now at level 2 – name change should succeed
    state.change_name(USER1_ADDRESS, 0, "Omega");

    let zombie = state.query_zombie(0);
    assert_eq!(zombie.name, ManagedBuffer::from("Omega"));
}

#[test]
fn test_change_name_not_owner_fails() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");
    state.level_up_zombie(USER1_ADDRESS, 0);
    state.level_up_zombie(USER1_ADDRESS, 0); // get to level 3 just in case

    state.change_name_expect_err(
        USER2_ADDRESS,
        0,
        "Hacker",
        "Only the owner of the zombie can perform this operation",
    );
}

#[test]
fn test_change_dna_requires_level_20() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");

    // Level 1 – should fail
    state.change_dna_expect_err(USER1_ADDRESS, 0, 1234567890u64, "Zombie is too low level");
}

#[test]
fn test_attack_not_owner_fails() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state.create_zombie(USER1_ADDRESS, "Alpha");
    state.create_zombie(USER2_ADDRESS, "Beta");

    // USER2 tries to attack using USER1's zombie (id 0)
    state.attack_expect_err(
        USER2_ADDRESS,
        0,
        1,
        "Only the owner of the zombie can perform this operation",
    );
}

#[test]
fn test_attack_updates_win_or_loss_count() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state
        .world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(0));

    state.create_zombie(USER1_ADDRESS, "Alpha");
    state.create_zombie(USER2_ADDRESS, "Beta");

    // Attack succeeds (outcome is random, but one count must change)
    state.attack(USER1_ADDRESS, 0, 1);

    let attacker = state.query_zombie(0);
    assert!(
        attacker.win_count + attacker.loss_count == 1,
        "Attacker must have exactly one win or one loss after the attack"
    );
}

#[test]
fn test_attack_win_creates_new_zombie_for_attacker() {
    let mut state = CryptoZombiesState::new();
    state.deploy();
    state
        .world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(0));

    state.create_zombie(USER1_ADDRESS, "Alpha");
    state.create_zombie(USER2_ADDRESS, "Beta");

    let initial_index = state.query_zombie_last_index();
    assert_eq!(initial_index, 2usize);

    state.attack(USER1_ADDRESS, 0, 1);

    let attacker = state.query_zombie(0);
    if attacker.win_count == 1 {
        // A win should have spawned a new zombie
        let new_index = state.query_zombie_last_index();
        assert_eq!(
            new_index, 3usize,
            "A winning attack should create a new zombie"
        );
    } else {
        // Loss: no new zombie is created
        let new_index = state.query_zombie_last_index();
        assert_eq!(
            new_index, 2usize,
            "A losing attack must not create a new zombie"
        );
    }
}

#[test]
fn test_withdraw_by_owner() {
    let mut state = CryptoZombiesState::new();
    state.deploy();

    // Owner can always call withdraw (even when collected_fees == 0)
    state.withdraw(OWNER_ADDRESS);
}

#[test]
fn test_withdraw_by_non_owner_fails() {
    let mut state = CryptoZombiesState::new();
    state.deploy();

    state.withdraw_expect_err(USER1_ADDRESS, "Endpoint can only be called by owner");
}

#[test]
fn test_set_crypto_kitties_address_by_owner() {
    let mut state = CryptoZombiesState::new();
    state.deploy();

    let kitty_sc = TestSCAddress::new("kitty-sc");
    state
        .world
        .account(kitty_sc)
        .nonce(1)
        .code(MxscPath::new("output/crypto-zombies.mxsc.json")); // any code – just sets up address

    state
        .world
        .tx()
        .id("set-kitty-address")
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::CryptoZombiesProxy)
        .set_crypto_kitties_sc_address(kitty_sc.to_address())
        .run();

    // Verify via storage view
    let stored: ManagedAddress<StaticApi> = state
        .world
        .query()
        .to(SC_ADDRESS)
        .typed(proxy::CryptoZombiesProxy)
        .crypto_kitties_sc_address()
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        stored,
        ManagedAddress::from(kitty_sc.to_address()),
        "Crypto Kitties address was not stored correctly"
    );
}

#[test]
fn test_set_crypto_kitties_address_non_owner_fails() {
    let mut state = CryptoZombiesState::new();
    state.deploy();

    state
        .world
        .tx()
        .from(USER1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::CryptoZombiesProxy)
        .set_crypto_kitties_sc_address(USER1_ADDRESS.to_address())
        .returns(ExpectError(4, "Endpoint can only be called by owner"))
        .run();
}

#[test]
fn test_multiple_users_each_create_one_zombie() {
    let mut state = CryptoZombiesState::new();
    state.deploy();

    state.create_zombie(USER1_ADDRESS, "Zombie1");
    state.create_zombie(USER2_ADDRESS, "Zombie2");

    let index = state.query_zombie_last_index();
    assert_eq!(index, 2usize);

    let z0 = state.query_zombie(0);
    let z1 = state.query_zombie(1);

    assert_eq!(z0.name, ManagedBuffer::from("Zombie1"));
    assert_eq!(z1.name, ManagedBuffer::from("Zombie2"));
}
