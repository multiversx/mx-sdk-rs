// Auto-generated blackbox tests from scenarios

use multiversx_sc_scenario::imports::*;

#[test]
fn pay_fee_and_fund_esdt_single_scen() {
    let mut world = ScenarioWorld::new();
    pay_fee_and_fund_esdt_single_scen_steps(&mut world);
}

pub fn pay_fee_and_fund_esdt_single_scen_steps(world: &mut ScenarioWorld) {
    whitelist_blacklist_fee_tokens_scen_steps(world);
    // Step 1: ScCall (id: pay-fee-and-fund-esdt-success)
    // Step 2: CheckState
}

#[test]
fn forward_scen() {
    let mut world = ScenarioWorld::new();
    forward_scen_steps(&mut world);
}

pub fn forward_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);
    // Step 1: ScCall (id: forward-fail)
    // Step 2: ScCall (id: deposit-fees-2)
    // Step 3: ScCall (id: forward-without-fees-ok)
    // Step 4: ScCall (id: deposit-fees-4)
    // Step 5: ScCall (id: forward-with-fees-fail)
    // Step 6: ScCall (id: forward-with-fees-ok)
    // Step 7: CheckState
}

#[test]
fn pay_fee_and_fund_esdt_multiple_scen() {
    let mut world = ScenarioWorld::new();
    pay_fee_and_fund_esdt_multiple_scen_steps(&mut world);
}

pub fn pay_fee_and_fund_esdt_multiple_scen_steps(world: &mut ScenarioWorld) {
    whitelist_blacklist_fee_tokens_scen_steps(world);
    // Step 1: ScCall (id: pay-fee-and-fund-esdt-fail)
    // Step 2: ScCall (id: pay-fee-and-fund-esdt-success)
    // Step 3: CheckState
}

#[test]
fn fund_egld_and_esdt_scen() {
    let mut world = ScenarioWorld::new();
    fund_egld_and_esdt_scen_steps(&mut world);
}

pub fn fund_egld_and_esdt_scen_steps(world: &mut ScenarioWorld) {
    set_accounts_scen_steps(world);
    // Step 1: ScCall (id: fail-fund)
    // Step 2: ScCall (id: deposit-fees-1)
    // Step 3: ScCall (id: fund-1)
    // Step 4: CheckState
    // Step 5: ScCall (id: deposit-fees-2)
    // Step 6: ScCall (id: fund-2)
    // Step 7: CheckState
    // Step 8: ScCall (id: fund-fail-3)
    // Step 9: ScCall (id: deposit-fees-3)
    // Step 10: ScCall (id: fund-2)
    // Step 11: CheckState
}

#[test]
fn claim_esdt_scen() {
    let mut world = ScenarioWorld::new();
    claim_esdt_scen_steps(&mut world);
}

pub fn claim_esdt_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);
    // Step 1: ScCall (id: claim2)
    // Step 2: SetState - set block
    // Step 3: ScCall (id: claim3-esdt)
    // Step 4: SetState - set block
    // Step 5: ScCall (id: claim4)
    // Step 6: ScCall (id: claim5-esdt)
    // Step 7: CheckState
}

#[test]
fn claim_egld_scen() {
    let mut world = ScenarioWorld::new();
    claim_egld_scen_steps(&mut world);
}

pub fn claim_egld_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);
    // Step 1: ScCall (id: claim2-fail)
    // Step 2: SetState - set block
    // Step 3: ScCall (id: claim3-egld-fail-expired)
    // Step 4: SetState - set block
    // Step 5: ScCall (id: claim4-fail)
    // Step 6: ScCall (id: claim5-egld)
    // Step 7: CheckState
}

#[test]
fn pay_fee_and_fund_egld_scen() {
    let mut world = ScenarioWorld::new();
    pay_fee_and_fund_egld_scen_steps(&mut world);
}

pub fn pay_fee_and_fund_egld_scen_steps(world: &mut ScenarioWorld) {
    whitelist_blacklist_fee_tokens_scen_steps(world);
    // Step 1: ScCall (id: pay-fee-and-fund-egld-fail)
    // Step 2: ScCall (id: pay-fee-and-fund-egld-success)
    // Step 3: CheckState
}

#[test]
fn claim_fees_scen() {
    let mut world = ScenarioWorld::new();
    claim_fees_scen_steps(&mut world);
}

pub fn claim_fees_scen_steps(world: &mut ScenarioWorld) {
    claim_egld_scen_steps(world);
    // Step 1: ScCall (id: claim-fees-fail)
    // Step 2: SetState - set block
    // Step 3: ScCall (id: claim-fees-ok)
    // Step 4: CheckState
}

#[test]
fn claim_multi_esdt_scen() {
    let mut world = ScenarioWorld::new();
    claim_multi_esdt_scen_steps(&mut world);
}

pub fn claim_multi_esdt_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);
    // Step 1: ScCall (id: claim2)
    // Step 2: SetState - set block
    // Step 3: ScCall (id: claim3-multi)
    // Step 4: SetState - set block
    // Step 5: ScCall (id: claim4)
    // Step 6: ScCall (id: claim5-multi)
    // Step 7: CheckState
}

#[test]
fn set_accounts_scen() {
    let mut world = ScenarioWorld::new();
    set_accounts_scen_steps(&mut world);
}

pub fn set_accounts_scen_steps(world: &mut ScenarioWorld) {
    // Step 0: SetState
    // Step 1: ScDeploy (id: deploy)
    // Step 2: CheckState
}

#[test]
fn withdraw_esdt_scen() {
    let mut world = ScenarioWorld::new();
    withdraw_esdt_scen_steps(&mut world);
}

pub fn withdraw_esdt_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);
    // Step 1: ScCall (id: withdraw-esdt-fail-1)
    // Step 2: CheckState
    // Step 3: ScCall (id: withdraw-esdt-fail-2)
    // Step 4: SetState - set block
    // Step 5: ScCall (id: withdraw-esdt-success)
    // Step 6: CheckState
}

#[test]
fn withdraw_egld_scen() {
    let mut world = ScenarioWorld::new();
    withdraw_egld_scen_steps(&mut world);
}

pub fn withdraw_egld_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);
    // Step 1: ScCall (id: withdraw-egld-fail-1)
    // Step 2: CheckState
    // Step 3: ScCall (id: withdraw-egld-fail-2)
    // Step 4: SetState - set block
    // Step 5: ScCall (id: withdraw-egld-success)
    // Step 6: CheckState
}

#[test]
fn withdraw_multi_esdt_scen() {
    let mut world = ScenarioWorld::new();
    withdraw_multi_esdt_scen_steps(&mut world);
}

pub fn withdraw_multi_esdt_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);
    // Step 1: ScCall (id: withdraw-esdt-1)
    // Step 2: CheckState
    // Step 3: ScCall (id: withdraw-esdt-2)
    // Step 4: SetState - set block
    // Step 5: ScCall (id: withdraw-esdt-3)
    // Step 6: CheckState
}

#[test]
fn whitelist_blacklist_fee_tokens_scen() {
    let mut world = ScenarioWorld::new();
    whitelist_blacklist_fee_tokens_scen_steps(&mut world);
}

pub fn whitelist_blacklist_fee_tokens_scen_steps(world: &mut ScenarioWorld) {
    set_accounts_scen_steps(world);
    // Step 1: ScCall (id: whitelist-success-1)
    // Step 2: ScCall (id: whitelist-success-2)
    // Step 3: ScCall (id: blacklist-success)
    // Step 4: CheckState
}

