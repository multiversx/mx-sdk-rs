use multiversx_chain_vm::schedule::{GasSchedule, GasScheduleVersion};

#[test]
fn new_struct_with_gas_schedule_test() {
    let gas_schedule_v8 = GasSchedule::new(GasScheduleVersion::V8);

    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_unreachable, 5);
    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_nop, 5);
    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_block, 5);

    let _gas_schedule_v1 = GasSchedule::new(GasScheduleVersion::V1);
    let _gas_schedule_v2 = GasSchedule::new(GasScheduleVersion::V2);
    let _gas_schedule_v3 = GasSchedule::new(GasScheduleVersion::V3);
    let _gas_schedule_v4 = GasSchedule::new(GasScheduleVersion::V4);
    let _gas_schedule_v5 = GasSchedule::new(GasScheduleVersion::V5);
    let _gas_schedule_v6 = GasSchedule::new(GasScheduleVersion::V6);
    let _gas_schedule_v7 = GasSchedule::new(GasScheduleVersion::V7);
}
