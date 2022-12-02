use std::{fs::File, io::Write};

pub(crate) fn write_newline(file: &mut File) {
    file.write_all(b"\n").unwrap();
}
