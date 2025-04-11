use multiversx_chain_vm::schedule::{GasSchedule, GasScheduleVersion};

#[test]
fn deserialize_test() {
    let gas_schedule_v8 = GasScheduleVersion::V8.load_gas_schedule();

    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_unreachable, 5);
    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_nop, 5);
    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_block, 5);
}

#[test]
fn serialize_deserialize_test() {
    let gas_schedule_v8 = GasScheduleVersion::V8.load_gas_schedule();
    let serialized = toml::to_string(&gas_schedule_v8).unwrap();
    let deserialized: GasSchedule = toml::from_str(&serialized).unwrap();

    assert_eq!(gas_schedule_v8, deserialized);
}
