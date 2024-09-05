use std::fs::File;

#[must_use]
pub(crate) fn create_file(proxy_file_name: &str) -> File {
    let file = format!("../{proxy_file_name}");

    File::create(file).expect("could not write proxy file")
}
