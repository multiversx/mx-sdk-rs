use std::fs::File;

#[must_use]
pub(crate) fn create_file(proxies_file_name: &str, overwrite: bool) -> File {
    let file = format!("../{proxies_file_name}");

    if overwrite {
        File::create(&file).unwrap()
    } else {
        match File::options().create_new(true).write(true).open(&file) {
            Ok(f) => f,
            Err(_) => panic!("{file} file already exists, --overwrite option was not provided"),
        }
    }
}
