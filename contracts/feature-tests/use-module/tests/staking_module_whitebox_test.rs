use multiversx_sc_modules::staking::StakingModule;
use multiversx_sc_scenario::imports::*;

const STAKING_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("STAKE-123456");
const INITIAL_BALANCE: u64 = 2_000_000;
const REQUIRED_STAKE_AMOUNT: u64 = 1_000_000;
const SLASH_AMOUNT: u64 = 600_000;
const QUORUM: usize = 3;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ALICE_ADDRESS: TestAddress = TestAddress::new("alice");
const BOB_ADDRESS: TestAddress = TestAddress::new("bob");
const CAROL_ADDRESS: TestAddress = TestAddress::new("carol");
const EVE_ADDRESS: TestAddress = TestAddress::new("eve");
const PAUL_ADDRESS: TestAddress = TestAddress::new("paul");
const SALLY_ADDRESS: TestAddress = TestAddress::new("sally");

const USE_MODULE_ADDRESS: TestSCAddress = TestSCAddress::new("use-module");
const USE_MODULE_PATH_EXPR: MxscPath = MxscPath::new("mxsc:output/use-module.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(USE_MODULE_PATH_EXPR, use_module::ContractBuilder);
    blockchain
}

#[test]
fn test_staking_module() {
    let mut world = world();

    world.account(OWNER_ADDRESS).nonce(1);
    world
        .account(ALICE_ADDRESS)
        .nonce(1)
        .esdt_balance(STAKING_TOKEN_ID, INITIAL_BALANCE);
    world
        .account(BOB_ADDRESS)
        .nonce(1)
        .esdt_balance(STAKING_TOKEN_ID, INITIAL_BALANCE);
    world
        .account(CAROL_ADDRESS)
        .nonce(1)
        .esdt_balance(STAKING_TOKEN_ID, INITIAL_BALANCE);
    world
        .account(EVE_ADDRESS)
        .nonce(1)
        .esdt_balance(STAKING_TOKEN_ID, INITIAL_BALANCE);
    world
        .account(PAUL_ADDRESS)
        .nonce(1)
        .esdt_balance(STAKING_TOKEN_ID, INITIAL_BALANCE);
    world
        .account(SALLY_ADDRESS)
        .nonce(1)
        .esdt_balance(STAKING_TOKEN_ID, INITIAL_BALANCE);

    // init
    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .raw_deploy()
        .code(USE_MODULE_PATH_EXPR)
        .new_address(USE_MODULE_ADDRESS)
        .returns(ReturnsNewBech32Address)
        .whitebox(use_module::contract_obj, |sc| {
            let mut whitelist = ManagedVec::new();
            whitelist.push(ALICE_ADDRESS.to_managed_address());
            whitelist.push(BOB_ADDRESS.to_managed_address());
            whitelist.push(CAROL_ADDRESS.to_managed_address());
            whitelist.push(PAUL_ADDRESS.to_managed_address());
            whitelist.push(SALLY_ADDRESS.to_managed_address());

            sc.init_staking_module(
                &EgldOrEsdtTokenIdentifier::esdt(STAKING_TOKEN_ID.to_token_identifier()),
                &BigUint::from(REQUIRED_STAKE_AMOUNT),
                &BigUint::from(SLASH_AMOUNT),
                QUORUM,
                &whitelist,
            );
        });

    assert_eq!(new_address.to_address(), USE_MODULE_ADDRESS.to_address());

    // try stake - not a board member
    world
        .tx()
        .from(EVE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(TestEsdtTransfer(STAKING_TOKEN_ID, 0, REQUIRED_STAKE_AMOUNT))
        .returns(ExpectError(4u64, "Only whitelisted members can stake"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.stake();
        });

    // stake half and try unstake
    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(TestEsdtTransfer(
            STAKING_TOKEN_ID,
            0,
            REQUIRED_STAKE_AMOUNT / 2,
        ))
        .whitebox(use_module::contract_obj, |sc| {
            sc.stake();
        });

    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Not enough stake"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.unstake(BigUint::from(REQUIRED_STAKE_AMOUNT / 4));
        });

    // bob and carol stake
    world
        .tx()
        .from(BOB_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(TestEsdtTransfer(STAKING_TOKEN_ID, 0, REQUIRED_STAKE_AMOUNT))
        .whitebox(use_module::contract_obj, |sc| {
            sc.stake();
        });

    world
        .tx()
        .from(CAROL_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(TestEsdtTransfer(STAKING_TOKEN_ID, 0, REQUIRED_STAKE_AMOUNT))
        .whitebox(use_module::contract_obj, |sc| {
            sc.stake();
        });

    world
        .tx()
        .from(PAUL_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(TestEsdtTransfer(STAKING_TOKEN_ID, 0, REQUIRED_STAKE_AMOUNT))
        .whitebox(use_module::contract_obj, |sc| {
            sc.stake();
        });

    world
        .tx()
        .from(SALLY_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(TestEsdtTransfer(STAKING_TOKEN_ID, 0, REQUIRED_STAKE_AMOUNT))
        .whitebox(use_module::contract_obj, |sc| {
            sc.stake();
        });

    // try vote slash, not enough stake
    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Not enough stake"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote_slash_member(BOB_ADDRESS.to_managed_address());
        });

    // try vote slash, slashed address not a board member
    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Voted user is not a staked board member"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote_slash_member(EVE_ADDRESS.to_managed_address());
        });

    // alice stake over max amount and withdraw surplus
    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(TestEsdtTransfer(STAKING_TOKEN_ID, 0, REQUIRED_STAKE_AMOUNT))
        .whitebox(use_module::contract_obj, |sc| {
            sc.stake();
            let alice_staked_amount = sc.staked_amount(&ALICE_ADDRESS.to_managed_address()).get();
            assert_eq!(alice_staked_amount, BigUint::from(1_500_000u64));
        });

    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.unstake(BigUint::from(500_000u64));
            let alice_staked_amount = sc.staked_amount(&ALICE_ADDRESS.to_managed_address()).get();
            assert_eq!(alice_staked_amount, BigUint::from(1_000_000u64));
        });

    world
        .check_account(ALICE_ADDRESS)
        .esdt_balance(STAKING_TOKEN_ID, BigUint::from(1_000_000u64));

    // alice vote to slash bob

    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote_slash_member(BOB_ADDRESS.to_managed_address());
            assert_eq!(
                sc.slashing_proposal_voters(&BOB_ADDRESS.to_managed_address())
                    .len(),
                1
            );
            assert!(sc
                .slashing_proposal_voters(&BOB_ADDRESS.to_managed_address())
                .contains(&ALICE_ADDRESS.to_managed_address()));
        });

    // bob vote to slash alice
    world
        .tx()
        .from(BOB_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote_slash_member(ALICE_ADDRESS.to_managed_address());
        });

    // try slash before quorum reached
    world
        .tx()
        .from(BOB_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Quorum not reached"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.slash_member(ALICE_ADDRESS.to_managed_address());
        });

    // paul vote to slash alice
    world
        .tx()
        .from(PAUL_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote_slash_member(ALICE_ADDRESS.to_managed_address());
        });

    // sally vote to slash alice
    world
        .tx()
        .from(SALLY_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote_slash_member(ALICE_ADDRESS.to_managed_address());
        });

    // sally cancels vote to slash alice
    world
        .tx()
        .from(SALLY_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.cancel_vote_slash_member(ALICE_ADDRESS.to_managed_address());
        });

    // carol vote
    world
        .tx()
        .from(CAROL_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote_slash_member(ALICE_ADDRESS.to_managed_address());

            assert_eq!(
                sc.slashing_proposal_voters(&ALICE_ADDRESS.to_managed_address())
                    .len(),
                3
            );
            assert!(sc
                .slashing_proposal_voters(&ALICE_ADDRESS.to_managed_address())
                .contains(&BOB_ADDRESS.to_managed_address()));
            assert!(sc
                .slashing_proposal_voters(&ALICE_ADDRESS.to_managed_address())
                .contains(&CAROL_ADDRESS.to_managed_address()));
            assert!(sc
                .slashing_proposal_voters(&ALICE_ADDRESS.to_managed_address())
                .contains(&PAUL_ADDRESS.to_managed_address()));
            assert!(!sc
                .slashing_proposal_voters(&ALICE_ADDRESS.to_managed_address())
                .contains(&SALLY_ADDRESS.to_managed_address()));
        });

    // slash alice
    world
        .tx()
        .from(BOB_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.slash_member(ALICE_ADDRESS.to_managed_address());

            assert_eq!(
                sc.staked_amount(&ALICE_ADDRESS.to_managed_address()).get(),
                BigUint::from(REQUIRED_STAKE_AMOUNT - SLASH_AMOUNT)
            );
            assert_eq!(sc.total_slashed_amount().get(), BigUint::from(SLASH_AMOUNT));
            assert!(sc
                .slashing_proposal_voters(&ALICE_ADDRESS.to_managed_address())
                .is_empty());
        });

    // alice try vote after slash
    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Not enough stake"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote_slash_member(BOB_ADDRESS.to_managed_address());
        });

    // alice try unstake the remaining tokens
    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Not enough stake"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.unstake(BigUint::from(400_000u64));
        });

    // alice remove from board members
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            // check alice's votes before slash
            assert!(sc
                .slashing_proposal_voters(&BOB_ADDRESS.to_managed_address())
                .contains(&ALICE_ADDRESS.to_managed_address()));

            sc.remove_board_member(&ALICE_ADDRESS.to_managed_address());

            assert_eq!(sc.user_whitelist().len(), 4);
            assert!(!sc
                .user_whitelist()
                .contains(&ALICE_ADDRESS.to_managed_address()));

            // alice's vote gets removed
            assert!(sc
                .slashing_proposal_voters(&BOB_ADDRESS.to_managed_address())
                .is_empty());
        });

    // alice unstake ok
    world
        .tx()
        .from(ALICE_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.unstake(BigUint::from(400_000u64));
        });

    world
        .check_account(ALICE_ADDRESS)
        .esdt_balance(STAKING_TOKEN_ID, INITIAL_BALANCE - SLASH_AMOUNT);
}
