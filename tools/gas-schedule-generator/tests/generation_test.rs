use gas_schedule_generator::generate_to_string;

const GAS_SCHEDULE_SECTIONS_FILE: &str =
    "../../chain/vm/src/schedule/gas_schedule_sections.rs";
const GENERATED_SECTIONS_FILE: &str = "generated_sections.rs";

#[test]
fn generation_test() {
    let generated = generate_to_string();
    std::fs::write(GENERATED_SECTIONS_FILE, &generated).unwrap();

    let on_disk = std::fs::read_to_string(GAS_SCHEDULE_SECTIONS_FILE).unwrap();
    assert_eq!(generated, on_disk);
}
