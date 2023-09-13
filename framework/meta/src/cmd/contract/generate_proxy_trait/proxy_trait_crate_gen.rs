use std::fs::File;

#[must_use]
pub(crate) fn create_and_get_lib_file(proxies_file_name: &str, overwrite: bool) -> File {
    let lib_path = format!("../{proxies_file_name}");
    if overwrite {
        File::create(&lib_path).unwrap()
    } else {
        match File::options().create_new(true).write(true).open(&lib_path) {
            Ok(f) => f,
            Err(_) => panic!("{lib_path} file already exists, --overwrite option for proxies was not provided"),
        }
    }
}
