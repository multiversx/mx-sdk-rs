mod op_gen_lib;

use std::{fs, path::Path};

fn main() {
    let target_path = "../../contracts/feature-tests/basic-features/src/big_num_operators.rs";
    let content = op_gen_lib::generate_big_int_operators_trait();
    if let Err(e) = fs::write(Path::new(target_path), content) {
        eprintln!("Failed to write to {}: {}", target_path, e);
        std::process::exit(1);
    } else {
        println!("Successfully rewrote {}", target_path);
    }
}
