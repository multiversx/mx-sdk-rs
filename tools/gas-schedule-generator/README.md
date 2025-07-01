# Gas schedule Rust structs generator

A small Rust program that generates Rust types based on the structure of a specific `gasSchedule` TOML file. The generated structs maintain the order of the fields, so that both serialization and deserialization are possible.


### Features

- Rust code generator
- TOML parser
- Customizable output path
- Small CLI 


### How to use

The program can be run easily using the CLI command `generate` in the project root, as such:

```bash
cargo run generate --toml-version 8
```

This command will generate gasSchedule section structs based on the `gasScheduleV8.toml` file content.

To specify a custom output path, create a file named `output_file_path.txt` in the project root. Inside this file, paste the full path where the generated file should be saved. If the file is not provided, the default output folder `output/` will be used.
