use std::fs;

pub const OUTPUT_WAT: &str = "single-import/wat";
pub const OUTPUT_WASM: &str = "single-import/wasm";

pub fn write_sc_files(hook_name: &str) {
    fs::create_dir_all(OUTPUT_WAT).unwrap();
    fs::create_dir_all(OUTPUT_WASM).unwrap();

    let sc_wat = generate_wat(hook_name);
    fs::write(
        format!("{OUTPUT_WAT}/single_import_{hook_name}.wat"),
        &sc_wat,
    )
    .unwrap();
    let sc_wasm = wat::parse_str(sc_wat.as_str()).unwrap();
    fs::write(
        format!("{OUTPUT_WASM}/single_import_{hook_name}.wasm"),
        sc_wasm,
    )
    .unwrap();
}

fn generate_wat(hook_name: &str) -> String {
    let mut sc_wat = String::new();
    sc_wat.push_str(WAT_1);
    let import_line = format!(r#"  (import "env" "{hook_name}" (func (type 1)))"#);
    sc_wat.push_str(&import_line);
    sc_wat.push_str(WAT_2);
    sc_wat
}

const WAT_1: &str = r#"
(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32)))
  (import "env" "getNumArguments" (func (;0;) (type 1)))
  (import "env" "signalError" (func (;1;) (type 2)))
  (import "env" "checkNoPayment" (func (;2;) (type 0)))
"#;
const WAT_2: &str = r#"
  (memory (;0;) 3)
  (global (;0;) i32 i32.const 131097)
  (global (;1;) i32 i32.const 131104)
  (export "memory" (memory 0))
  (export "init" (func 3))
  (export "callBack" (func 4))
  (export "upgrade" (func 3))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1))
  (func (;3;) (type 0)
    call 2
    call 0
    if ;; label = @1
      i32.const 131072
      i32.const 25
      call 1
      unreachable
    end
  )
  (func (;4;) (type 0))
  (data (;0;) (i32.const 131072) "wrong number of arguments")
)
"#;
