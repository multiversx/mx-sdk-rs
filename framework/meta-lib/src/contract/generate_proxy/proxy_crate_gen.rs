use std::{fs::File, path::Path};

#[must_use]
pub(crate) fn create_file(proxy_file_path: &Path) -> File {
    let path = Path::new("..").join(proxy_file_path);

    File::create(path).expect("could not write proxy file")
}
