const GENERATED_SECTIONS_FILE: &str = "generated_sections.rs";

#[test]
fn generation_test() {
    let generated = gas_schedule_generator::generate_to_string();
    std::fs::write(GENERATED_SECTIONS_FILE, &generated).unwrap();

    let on_disk = std::fs::read_to_string(gas_schedule_generator::SECTIONS_FILE_PATH).unwrap();
    assert_eq!(
        generated, on_disk,
        "Generated content does not match the content on disk. Please run `cargo run` in `tools/gas-schedule-generator` to regenerate the file."
    );
}
