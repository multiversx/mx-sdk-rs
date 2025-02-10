use std::{
    fs::File,
    io::{self, Write},
};

const MIN: usize = 1;
const MAX: usize = 128;

/// Generates the payload_add! macros in the ManagedVecItem implem,entation.
///
/// TODO: remove once generic const expressions are stabilized in Rust.
fn main() -> io::Result<()> {
    let mut file = File::create("output.rs")?;

    // Generate add_sub_const_decimals! macro combinations
    writeln!(file, "payload_ops! {{")?;

    for dec1 in MIN..=MAX {
        write!(file, "    ({dec1}usize")?;

        for decn in (MIN..dec1).rev() {
            write!(file, ", {decn}usize")?;
        }

        writeln!(file, ")")?;
    }

    writeln!(file, "}}")?;

    Ok(())
}
