#[cfg(test)]
pub mod tests {
    use std::{
        collections::{HashMap, HashSet},
        path::{Path, PathBuf},
    };

    use wat::Parser;

    use crate::tools::{panic_report::PanicReport, wasm_extractor::populate_wasm_info, WasmInfo};

    const ADDER_WITH_ERR_IN_VIEW: &str = r#"
(module $adder_wasm.wasm
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func))
  (type (;5;) (func (param i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32) (result i32)))
  (import "env" "bigIntGetUnsignedArgument" (func $bigIntGetUnsignedArgument (;0;) (type 0)))
  (import "env" "getNumArguments" (func $getNumArguments (;1;) (type 1)))
  (import "env" "signalError" (func $signalError (;2;) (type 0)))
  (import "env" "mBufferFromBigIntUnsigned" (func $mBufferFromBigIntUnsigned (;3;) (type 2)))
  (import "env" "mBufferStorageStore" (func $mBufferStorageStore (;4;) (type 2)))
  (import "env" "mBufferStorageLoad" (func $mBufferStorageLoad (;5;) (type 2)))
  (import "env" "mBufferToBigIntUnsigned" (func $mBufferToBigIntUnsigned (;6;) (type 2)))
  (import "env" "mBufferSetBytes" (func $mBufferSetBytes (;7;) (type 3)))
  (import "env" "checkNoPayment" (func $checkNoPayment (;8;) (type 4)))
  (import "env" "bigIntFinishUnsigned" (func $bigIntFinishUnsigned (;9;) (type 5)))
  (import "env" "bigIntAdd" (func $bigIntAdd (;10;) (type 6)))
  (func $_ZN13multiversx_sc2io16arg_nested_tuple15load_single_arg17hcaef680f5560198bE (;11;) (type 1) (result i32)
    (local i32)
    i32.const 0
    call $_ZN26multiversx_sc_wasm_adapter3api13managed_types19static_var_api_node11next_handle17h315bf8f89178ffe1E
    local.tee 0
    call $bigIntGetUnsignedArgument
    local.get 0
  )
  (func $_ZN26multiversx_sc_wasm_adapter3api13managed_types19static_var_api_node11next_handle17h315bf8f89178ffe1E (;12;) (type 1) (result i32)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=131100
    i32.const -1
    i32.add
    local.tee 0
    i32.store offset=131100
    local.get 0
  )
  (func $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17h8cbbe81aa680cf46E (;13;) (type 5) (param i32)
    block ;; label = @1
      call $getNumArguments
      local.get 0
      i32.ne
      br_if 0 (;@1;)
      return
    end
    i32.const 131072
    i32.const 25
    call $signalError
    unreachable
  )
  (func $_ZN13multiversx_sc7storage7mappers19single_value_mapper31SingleValueMapper$LT$SA$C$T$GT$3set17h00fe05a7d5154b39E (;14;) (type 0) (param i32 i32)
    (local i32)
    call $_ZN26multiversx_sc_wasm_adapter3api13managed_types19static_var_api_node11next_handle17h315bf8f89178ffe1E
    local.tee 2
    local.get 1
    call $mBufferFromBigIntUnsigned
    drop
    local.get 0
    local.get 2
    call $mBufferStorageStore
    drop
  )
  (func $_ZN13multiversx_sc7storage7mappers19single_value_mapper35SingleValueMapper$LT$SA$C$T$C$A$GT$3get17hb9977d4c8d45f0d8E (;15;) (type 7) (param i32) (result i32)
    (local i32)
    local.get 0
    call $_ZN26multiversx_sc_wasm_adapter3api13managed_types19static_var_api_node11next_handle17h315bf8f89178ffe1E
    local.tee 1
    call $mBufferStorageLoad
    drop
    local.get 1
    call $_ZN26multiversx_sc_wasm_adapter3api13managed_types19static_var_api_node11next_handle17h315bf8f89178ffe1E
    local.tee 0
    call $mBufferToBigIntUnsigned
    drop
    local.get 0
  )
  (func $_ZN34_$LT$C$u20$as$u20$adder..Adder$GT$3sum17h7cc7cc0602a1f97fE (;16;) (type 1) (result i32)
    (local i32)
    call $_ZN26multiversx_sc_wasm_adapter3api13managed_types19static_var_api_node11next_handle17h315bf8f89178ffe1E
    local.tee 0
    i32.const 131097
    i32.const 3
    call $mBufferSetBytes
    drop
    local.get 0
  )
  (func $init (;17;) (type 4)
    (local i32)
    call $checkNoPayment
    i32.const 1
    call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17h8cbbe81aa680cf46E
    call $_ZN13multiversx_sc2io16arg_nested_tuple15load_single_arg17hcaef680f5560198bE
    local.set 0
    call $_ZN34_$LT$C$u20$as$u20$adder..Adder$GT$3sum17h7cc7cc0602a1f97fE
    local.get 0
    call $_ZN13multiversx_sc7storage7mappers19single_value_mapper31SingleValueMapper$LT$SA$C$T$GT$3set17h00fe05a7d5154b39E
  )
  (func $getSum (;18;) (type 4)
    call $checkNoPayment
    i32.const 0
    call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17h8cbbe81aa680cf46E
    call $_ZN34_$LT$C$u20$as$u20$adder..Adder$GT$3sum17h7cc7cc0602a1f97fE
    call $_ZN13multiversx_sc7storage7mappers19single_value_mapper35SingleValueMapper$LT$SA$C$T$C$A$GT$3get17hb9977d4c8d45f0d8E
    call $bigIntFinishUnsigned
  )
  (func $add (;19;) (type 4)
    (local i32 i32 i32)
    call $checkNoPayment
    i32.const 1
    call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17h8cbbe81aa680cf46E
    call $_ZN13multiversx_sc2io16arg_nested_tuple15load_single_arg17hcaef680f5560198bE
    local.set 0
    call $_ZN34_$LT$C$u20$as$u20$adder..Adder$GT$3sum17h7cc7cc0602a1f97fE
    local.tee 1
    call $_ZN13multiversx_sc7storage7mappers19single_value_mapper35SingleValueMapper$LT$SA$C$T$C$A$GT$3get17hb9977d4c8d45f0d8E
    local.tee 2
    local.get 2
    local.get 0
    call $bigIntAdd
    local.get 1
    local.get 2
    call $_ZN13multiversx_sc7storage7mappers19single_value_mapper31SingleValueMapper$LT$SA$C$T$GT$3set17h00fe05a7d5154b39E
  )
  (func $callBack (;20;) (type 4))
  (table (;0;) 1 1 funcref)
  (memory (;0;) 3)
  (global $__stack_pointer (;0;) (mut i32) i32.const 131072)
  (global (;1;) i32 i32.const 131104)
  (global (;2;) i32 i32.const 131104)
  (export "memory" (memory 0))
  (export "init" (func $init))
  (export "getSum" (func $getSum))
  (export "add" (func $add))
  (export "callBack" (func $callBack))
  (export "upgrade" (func $init))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data $.rodata (;0;) (i32.const 131072) "wrong number of argumentssum")
  (data $.data (;1;) (i32.const 131100) "8\ff\ff\ff")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.80.1 (3f5fd8dd4 2024-08-06)")
  )
  (@custom "target_features" (after data) "\02+\0fmutable-globals+\08sign-ext")
)
"#;

    const EMPTY_WITH_FAIL_ALLOCATOR: &str = r#"
(module $empty_wasm.wasm
	(type (;0;) (func (result i32)))
	(type (;1;) (func (param i32 i32)))
	(type (;2;) (func))
	(import "env" "getNumArguments" (func $getNumArguments (;0;) (type 0)))
	(import "env" "signalError" (func $signalError (;1;) (type 1)))
	(import "env" "checkNoPayment" (func $checkNoPayment (;2;) (type 2)))
	(func $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE (;3;) (type 2)
		block ;; label = @1
			call $getNumArguments
			br_if 0 (;@1;)
			return
		end
		i32.const 131072
		i32.const 25
		call $signalError
		unreachable
	)
	(func $__rust_alloc (;4;) (type 2)
		call $_ZN122_$LT$multiversx_sc_wasm_adapter..wasm_alloc..fail_allocator..FailAllocator$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17hf044a79dd04bdcb1E
		unreachable
	)
	(func $_ZN122_$LT$multiversx_sc_wasm_adapter..wasm_alloc..fail_allocator..FailAllocator$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17hf044a79dd04bdcb1E (;5;) (type 2)
		call $_ZN26multiversx_sc_wasm_adapter10wasm_alloc14fail_allocator29signal_allocation_not_allowed17hb15b243b5f851b48E
		unreachable
	)
	(func $init (;6;) (type 2)
		call $checkNoPayment
		call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE
		i32.const 0
		i32.load8_u offset=131124
		drop
		call $__rust_alloc
		unreachable
	)
	(func $upgrade (;7;) (type 2)
		call $checkNoPayment
		call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE
	)
	(func $callBack (;8;) (type 2))
	(func $_ZN26multiversx_sc_wasm_adapter10wasm_alloc14fail_allocator29signal_allocation_not_allowed17hb15b243b5f851b48E (;9;) (type 2)
		i32.const 131097
		i32.const 27
		call $signalError
		unreachable
	)
	(memory (;0;) 3)
	(global $__stack_pointer (;0;) (mut i32) i32.const 131072)
	(global (;1;) i32 i32.const 131125)
	(global (;2;) i32 i32.const 131136)
	(export "memory" (memory 0))
	(export "init" (func $init))
	(export "upgrade" (func $upgrade))
	(export "callBack" (func $callBack))
	(export "__data_end" (global 1))
	(export "__heap_base" (global 2))
	(data $.rodata (;0;) (i32.const 131072) "wrong number of argumentsmemory allocation forbidden")
)
"#;

    const EMPTY_WITH_MEM_GROW: &str = r#"
(module $empty_wasm.wasm
	(type (;0;) (func (result i32)))
	(type (;1;) (func (param i32 i32)))
	(type (;2;) (func))
	(type (;3;) (func (param i64)))
	(import "env" "getNumArguments" (func $getNumArguments (;0;) (type 0)))
	(import "env" "signalError" (func $signalError (;1;) (type 1)))
	(import "env" "checkNoPayment" (func $checkNoPayment (;2;) (type 2)))
	(import "env" "smallIntFinishUnsigned" (func $smallIntFinishUnsigned (;3;) (type 3)))
	(func $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE (;4;) (type 2)
		block ;; label = @1
			call $getNumArguments
			br_if 0 (;@1;)
			return
		end
		i32.const 131072
		i32.const 25
		call $signalError
		unreachable
	)
	(func $rust_begin_unwind (;5;) (type 2)
		call $_ZN26multiversx_sc_wasm_adapter5panic9panic_fmt17he68b14ffa9b6b21eE
		unreachable
	)
	(func $_ZN26multiversx_sc_wasm_adapter5panic9panic_fmt17he68b14ffa9b6b21eE (;6;) (type 2)
		i32.const 131097
		i32.const 14
		call $signalError
		unreachable
	)
	(func $init (;7;) (type 2)
		(local i32 i32)
		call $checkNoPayment
		call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE
		i32.const 0
		i32.load8_u offset=131120
		drop
		block ;; label = @1
			i32.const 0
			i32.load offset=131112
			local.tee 0
			i32.const 3
			i32.and
			i32.eqz
			br_if 0 (;@1;)
			i32.const 0
			local.get 0
			i32.const -4
			i32.and
			i32.const 4
			i32.add
			local.tee 0
			i32.store offset=131112
		end
		block ;; label = @1
			local.get 0
			i32.const 4
			i32.add
			local.tee 1
			i32.const 0
			i32.load offset=131116
			i32.le_u
			br_if 0 (;@1;)
			i32.const 1
			memory.grow
			local.set 0
			i32.const 0
			i32.load offset=131116
			local.set 1
			i32.const 0
			local.get 0
			i32.const 16
			i32.shl
			local.tee 0
			i32.const 65536
			i32.add
			i32.store offset=131116
			i32.const 0
			i32.load offset=131112
			local.get 0
			local.get 0
			local.get 1
			i32.eq
			select
			local.tee 0
			i32.const 4
			i32.add
			local.set 1
		end
		i32.const 0
		local.get 1
		i32.store offset=131112
		block ;; label = @1
			local.get 0
			i32.eqz
			br_if 0 (;@1;)
			local.get 0
			i32.const 42
			i32.store
			i64.const 1
			call $smallIntFinishUnsigned
			return
		end
		call $_ZN5alloc5alloc18handle_alloc_error17he71533634a7a5ac5E
		unreachable
	)
	(func $_ZN5alloc5alloc18handle_alloc_error17he71533634a7a5ac5E (;8;) (type 2)
		call $__rust_alloc_error_handler
		unreachable
	)
	(func $upgrade (;9;) (type 2)
		call $checkNoPayment
		call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE
	)
	(func $callBack (;10;) (type 2))
	(func $__rust_alloc_error_handler (;11;) (type 2)
		call $__rdl_oom
		unreachable
	)
	(func $__rdl_oom (;12;) (type 2)
		call $_ZN4core9panicking18panic_nounwind_fmt17h21a92179d680342aE
		unreachable
	)
	(func $_ZN4core9panicking18panic_nounwind_fmt17h21a92179d680342aE (;13;) (type 2)
		call $rust_begin_unwind
		unreachable
	)
	(memory (;0;) 3)
	(global $__stack_pointer (;0;) (mut i32) i32.const 131072)
	(global (;1;) i32 i32.const 131121)
	(global (;2;) i32 i32.const 131136)
	(export "memory" (memory 0))
	(export "init" (func $init))
	(export "upgrade" (func $upgrade))
	(export "callBack" (func $callBack))
	(export "__data_end" (global 1))
	(export "__heap_base" (global 2))
	(data $.rodata (;0;) (i32.const 131072) "wrong number of argumentspanic occurred")
)
"#;

    const EMPTY_DBG_WAT: &str = r#"
(module $empty_wasm.wasm
	(type (;0;) (func (result i32)))
	(type (;1;) (func (param i32 i32)))
	(type (;2;) (func))
	(type (;3;) (func (param i64)))
	(import "env" "getNumArguments" (func $getNumArguments (;0;) (type 0)))
	(import "env" "signalError" (func $signalError (;1;) (type 1)))
	(import "env" "checkNoPayment" (func $checkNoPayment (;2;) (type 2)))
	(import "env" "smallIntFinishUnsigned" (func $smallIntFinishUnsigned (;3;) (type 3)))
	(func $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE (;4;) (type 2)
		block ;; label = @1
			call $getNumArguments
			br_if 0 (;@1;)
			return
		end
		i32.const 131072
		i32.const 25
		call $signalError
		unreachable
	)
	(func $init (;5;) (type 2)
		call $checkNoPayment
		call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE
		i64.const 64
		call $smallIntFinishUnsigned
	)
	(func $upgrade (;6;) (type 2)
		call $checkNoPayment
		call $_ZN13multiversx_sc2io16arg_nested_tuple22check_num_arguments_eq17hd731b4a37a0ab09aE
	)
	(func $callBack (;7;) (type 2))
	(memory (;0;) 3)
	(global $__stack_pointer (;0;) (mut i32) i32.const 131072)
	(global (;1;) i32 i32.const 131097)
	(global (;2;) i32 i32.const 131104)
	(export "memory" (memory 0))
	(export "init" (func $init))
	(export "upgrade" (func $upgrade))
	(export "callBack" (func $callBack))
	(export "__data_end" (global 1))
	(export "__heap_base" (global 2))
	(data $.rodata (;0;) (i32.const 131072) "wrong number of arguments")
)
"#;

    #[test]
    fn test_empty() {
        if let Ok(content) = Parser::new().parse_bytes(None, EMPTY_DBG_WAT.as_bytes()) {
            let wasm_info = populate_wasm_info(Path::new(""), &content, false, None, &[])
                .expect("Unable to parse WASM content.");
            assert!(!wasm_info.memory_grow_flag);
            assert!(!wasm_info.report.has_allocator);
            assert_eq!(
                PanicReport::None.to_string(),
                wasm_info.report.has_panic.to_string()
            );
        }
    }

    #[test]
    fn test_empty_with_mem_grow() {
        if let Ok(content) = Parser::new().parse_bytes(None, EMPTY_WITH_MEM_GROW.as_bytes()) {
            let wasm_info = populate_wasm_info(Path::new(""), &content, false, None, &[])
                .expect("Unable to parse WASM content.");
            assert!(wasm_info.memory_grow_flag);
            assert!(!wasm_info.report.has_allocator);
            assert_eq!(
                PanicReport::WithoutMessage.to_string(),
                wasm_info.report.has_panic.to_string()
            );
        }
    }

    #[test]
    fn test_empty_with_fail_allocator() {
        if let Ok(content) = Parser::new().parse_bytes(None, EMPTY_WITH_FAIL_ALLOCATOR.as_bytes()) {
            let wasm_info = populate_wasm_info(Path::new(""), &content, false, None, &[])
                .expect("Unable to parse WASM content.");
            assert!(!wasm_info.memory_grow_flag);
            assert!(wasm_info.report.has_allocator);
            assert_eq!(
                PanicReport::None.to_string(),
                wasm_info.report.has_panic.to_string()
            );
        }
    }

    #[test]
    fn test_adder_with_write_op_in_view() {
        let view_endpoints: Vec<&str> = Vec::from(["getSum", "add"]);

        let expected_view_index: HashMap<String, usize> =
            HashMap::from([("getSum".to_string(), 18), ("add".to_string(), 19)]);
        let expected_write_index_functions: HashSet<usize> = HashSet::from([4, 19, 14]);
        let expected_call_graph: HashMap<usize, HashSet<usize>> = HashMap::from([
            (0, HashSet::new()),
            (1, HashSet::new()),
            (2, HashSet::new()),
            (3, HashSet::new()),
            (4, HashSet::new()),
            (5, HashSet::new()),
            (6, HashSet::new()),
            (7, HashSet::new()),
            (8, HashSet::new()),
            (9, HashSet::new()),
            (10, HashSet::new()),
            (11, HashSet::from([12, 0])),
            (12, HashSet::new()),
            (13, HashSet::from([1, 2])),
            (14, HashSet::from([12, 3, 4])),
            (15, HashSet::from([12, 5, 6])),
            (16, HashSet::from([12, 7])),
            (17, HashSet::from([8, 13, 11, 16, 14])),
            (18, HashSet::from([8, 13, 16, 15, 9])),
            (19, HashSet::from([8, 13, 11, 16, 15, 10, 14])),
            (20, HashSet::new()),
        ]);

        if let Ok(content) = Parser::new().parse_bytes(None, ADDER_WITH_ERR_IN_VIEW.as_bytes()) {
            let wasm_info =
                populate_wasm_info(Path::new(""), &content, false, None, &view_endpoints)
                    .expect("Unable to parse WASM content.");

            assert_eq!(
                expected_write_index_functions,
                wasm_info.write_index_functions
            );
            assert_eq!(expected_call_graph, wasm_info.call_graph);
            assert_eq!(expected_view_index, wasm_info.view_endpoints);
        }
    }

    #[test]
    fn test_data_drop() {
        // let data_drop_wat = wasm_to_wat("src/tools/forbidden-opcodes/data-drop.wasm", "");
        WasmInfo::extract_wasm_info(
            &PathBuf::from("src/tools/forbidden-opcodes/data-drop.wasm"),
            false,
            None,
            &[],
        );
    }
}
