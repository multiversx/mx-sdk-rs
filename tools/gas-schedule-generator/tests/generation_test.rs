use std::path::Path;

use gas_schedule_generator::generate_file_content;
use multiversx_chain_vm::schedule::GasScheduleVersion;

const VALID_FILE_CONTENT_NAME: &str = "valid_generated.txt";
const NEWLY_GENERATED_CONTENT_NAME: &str = "sections.rs";

#[test]

fn generation_test() {
    let gas_schedule_v8_content = GasScheduleVersion::V8;
    let valid_generated =
        std::fs::read_to_string(Path::new("tests").join(VALID_FILE_CONTENT_NAME)).unwrap();

    generate_file_content(gas_schedule_v8_content as u16);
    let newly_generated =
        std::fs::read_to_string(Path::new("output").join(NEWLY_GENERATED_CONTENT_NAME)).unwrap();

    assert_eq!(newly_generated, valid_generated);
}
