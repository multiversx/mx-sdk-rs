use std::{
    fs::File,
    io::{self, Write},
};

const MIN: usize = 1;
const MAX_X: usize = 48;
const MAX_Y: usize = 128;

/// Generates the payload_add! macros in the ManagedVecItem implem,entation.
/// 
/// TODO: remove once generic const expressions are stabilized in Rust.
fn main() -> io::Result<()> {
    let mut file = File::create("output.rs")?;

    // Generate add_sub_const_decimals! macro combinations
    for x in MIN..=MAX_X {
        for y in MIN..=MAX_Y {
            let sum = x + y;
            writeln!(
                file,
                "payload_add!({}usize, {}usize, {}usize);",
                x, y, sum
            )?;
        }
    }

    Ok(())
}
