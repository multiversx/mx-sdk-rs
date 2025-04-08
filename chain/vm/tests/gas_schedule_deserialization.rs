use multiversx_chain_vm::schedule::{GasSchedule, GasScheduleVersion};

#[test]
fn new_struct_with_gas_schedule_test() {
    let gas_schedule = GasSchedule::new(GasScheduleVersion::V8);

    assert_eq!(gas_schedule.wasm_opcode_cost.opcode_unreachable, 5);
    assert_eq!(gas_schedule.wasm_opcode_cost.opcode_nop, 5);
    assert_eq!(gas_schedule.wasm_opcode_cost.opcode_block, 5);
}
