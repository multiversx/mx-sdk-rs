use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn ping_pong_call_get_user_addresses_go() {
    world().run("scenarios/ping-pong-call-get-user-addresses.scen.json");
}

#[test]
fn ping_pong_call_ping_go() {
    world().run("scenarios/ping-pong-call-ping.scen.json");
}

#[test]
fn ping_pong_call_ping_after_deadline_go() {
    world().run("scenarios/ping-pong-call-ping-after-deadline.scen.json");
}

#[test]
fn ping_pong_call_ping_before_activation_go() {
    world().run("scenarios/ping-pong-call-ping-before-activation.scen.json");
}

#[test]
fn ping_pong_call_ping_before_beginning_go() {
    world().run("scenarios/ping-pong-call-ping-before-beginning.scen.json");
}

#[test]
fn ping_pong_call_ping_second_user_go() {
    world().run("scenarios/ping-pong-call-ping-second-user.scen.json");
}

#[test]
fn ping_pong_call_ping_twice_go() {
    world().run("scenarios/ping-pong-call-ping-twice.scen.json");
}

#[test]
fn ping_pong_call_ping_wrong_ammount_go() {
    world().run("scenarios/ping-pong-call-ping-wrong-ammount.scen.json");
}

#[test]
fn ping_pong_call_pong_go() {
    world().run("scenarios/ping-pong-call-pong.scen.json");
}

#[test]
fn ping_pong_call_pong_all_go() {
    world().run("scenarios/ping-pong-call-pong-all.scen.json");
}

#[test]
fn ping_pong_call_pong_all_after_pong_go() {
    world().run("scenarios/ping-pong-call-pong-all-after-pong.scen.json");
}

#[test]
fn ping_pong_call_pong_all_interrupted_1_go() {
    world().run("scenarios/ping-pong-call-pong-all-interrupted-1.scen.json");
}

#[test]
fn ping_pong_call_pong_all_interrupted_2_go() {
    world().run("scenarios/ping-pong-call-pong-all-interrupted-2.scen.json");
}

#[test]
fn ping_pong_call_pong_before_deadline_go() {
    world().run("scenarios/ping-pong-call-pong-before-deadline.scen.json");
}

#[test]
fn ping_pong_call_pong_twice_go() {
    world().run("scenarios/ping-pong-call-pong-twice.scen.json");
}

#[test]
fn ping_pong_call_pong_without_ping_go() {
    world().run("scenarios/ping-pong-call-pong-without-ping.scen.json");
}

#[test]
fn ping_pong_init_go() {
    world().run("scenarios/ping-pong-init.scen.json");
}
