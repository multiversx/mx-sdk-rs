use multiversx_chain_vm::schedule::{GasSchedule, GasScheduleVersion};

#[test]
fn deserialize_test() {
    let gas_schedule_v8 = GasSchedule::new(GasScheduleVersion::V8);

    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_unreachable, 5);
    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_nop, 5);
    assert_eq!(gas_schedule_v8.wasm_opcode_cost.opcode_block, 5);
}

#[test]
fn serialize_deserialize_test() {
    let gas_schedule_v8 = GasSchedule::new(GasScheduleVersion::V8);
    let serialized = toml::to_string(&gas_schedule_v8).unwrap();
    let deserialized: GasSchedule = toml::from_str(&serialized).unwrap();

    assert_eq!(gas_schedule_v8, deserialized);
}
