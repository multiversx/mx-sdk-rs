use std::{env, fs};

fn print_mandos_tests(names: Vec<String>) {
	for name in names.iter() {
		print!(
			"
#[test]
fn {}() {{
    parse_execute_mandos(\"mandos/{}.scen.json\", &contract_map());
}}
",
			name.replace('-', "_").to_lowercase(),
			name
		);
	}
}

fn split_file_name(name: String, separator: &str) -> Vec<String> {
	let splitted_name = name.split(separator);
	let collection: Vec<&str> = splitted_name.collect();
	let mut converted_collection: Vec<String> = Vec::new();

	for item in collection {
		converted_collection.push(String::from(item));
	}

	return converted_collection;
}

fn read_dirs(path: &str) -> Vec<String> {
	let paths = fs::read_dir(path).unwrap();
	let mut names: Vec<String> = Vec::new();

	for dir in paths {
		let dir_abs_path =
			String::from(dir.unwrap().path().into_os_string().into_string().unwrap());
		let mut splitted_files_name: Vec<String> = split_file_name(dir_abs_path, "/");
		let files_name_with_extension = splitted_files_name.pop().unwrap();
		splitted_files_name = split_file_name(files_name_with_extension, ".");
		let files_names = String::from(splitted_files_name.first().unwrap());
		names.push(files_names);
	}

	names.sort();
	names
}

/// Example run:
/// `cargo run ../../contracts/examples/crowdfunding-erc20/mandos`
fn main() {
	let args: Vec<String> = env::args().collect();
	let files_path = &args[1];

	let names = read_dirs(files_path);

	print_mandos_tests(names);
}
