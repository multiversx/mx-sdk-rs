#[cfg(test)]
pub mod tests {
    use std::{
        collections::{BTreeMap, BTreeSet, HashMap, HashSet},
        path::PathBuf,
        vec,
    };

    use wat::Parser;

    use crate::tools::{
        panic_report::PanicReport,
        wasm_extractor::extractor::{get_view_endpoints, WasmInfo},
        OpcodeVersion,
    };

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

    const CALL_FUNC_WITH_MEMORY_FILL: &str = r#"
(module
    (memory 1)
    (type $fill_type (func (param i32 i32 i32)))
    (type $copy_type (func (param i32 i32 i32)))
    (table 2 funcref)
    (func $fill_memory (type $fill_type)
        (param $offset i32) (param $value i32) (param $len i32)
        local.get $offset
        local.get $value
        local.get $len
        memory.fill
    )
    (func $copy_memory (type $copy_type)
        (param $dst i32) (param $src i32) (param $len i32)
        local.get $dst
        local.get $src
        local.get $len
        memory.copy
    )
    (elem (i32.const 0) $fill_memory $copy_memory)
    (func (export "call_fill")
        i32.const 0    ;; offset
        i32.const 42   ;; value
        i32.const 10   ;; length
        call $fill_memory
    )
    (func (export "call_fill_indirect")
        i32.const 0    ;; offset
        i32.const 99   ;; value
        i32.const 5    ;; length
        i32.const 0    ;; table index
        call_indirect (type $fill_type)
    )
    (func (export "call_copy_indirect")
        i32.const 0    ;; dst
        i32.const 20   ;; src
        i32.const 5    ;; length
        i32.const 1    ;; table index (copy_memory)
        call_indirect (type $copy_type)
    )
    
)
"#;

    #[test]
    fn test_empty() {
        if let Ok(content) = Parser::new().parse_bytes(None, EMPTY_DBG_WAT.as_bytes()) {
            let wasm_info = WasmInfo::default()
                .add_wasm_data(&content)
                .populate_wasm_info(false, None, OpcodeVersion::V1)
                .expect("Unable to parse WASM content.");
            assert!(!wasm_info.report.memory_grow_flag);
            assert!(!wasm_info.report.code.has_allocator);
            assert_eq!(
                PanicReport::None.to_string(),
                wasm_info.report.code.has_panic.to_string()
            );
        }
    }

    #[test]
    fn test_empty_with_mem_grow() {
        if let Ok(content) = Parser::new().parse_bytes(None, EMPTY_WITH_MEM_GROW.as_bytes()) {
            let wasm_info = WasmInfo::default()
                .add_wasm_data(&content)
                .populate_wasm_info(false, None, OpcodeVersion::V1)
                .expect("Unable to parse WASM content.");
            assert!(wasm_info.report.memory_grow_flag);
            assert!(!wasm_info.report.code.has_allocator);
            assert_eq!(
                PanicReport::WithoutMessage.to_string(),
                wasm_info.report.code.has_panic.to_string()
            );
        }
    }

    #[test]
    fn test_empty_with_fail_allocator() {
        if let Ok(content) = Parser::new().parse_bytes(None, EMPTY_WITH_FAIL_ALLOCATOR.as_bytes()) {
            let wasm_info = WasmInfo::default()
                .add_endpoints(&HashMap::from([("init", false)]))
                .add_wasm_data(&content)
                .populate_wasm_info(false, None, OpcodeVersion::V1)
                .expect("Unable to parse WASM content.");
            assert!(!wasm_info.report.memory_grow_flag);
            assert!(wasm_info.report.code.has_allocator);
            assert_eq!(
                PanicReport::None.to_string(),
                wasm_info.report.code.has_panic.to_string()
            );
        }
    }

    // should trigger in terminal warning: "Write storage operation in VIEW endpoint: add"
    #[test]
    fn test_adder_with_write_op_in_view() {
        let expected_view_index: HashMap<&str, usize> =
            HashMap::from([("getSum", 18), ("add", 19)]);
        let expected_write_index_functions: HashSet<usize> = HashSet::from([4, 19, 14]);

        let expected_call_graph = vec![
            (0, vec![]),
            (1, vec![]),
            (2, vec![]),
            (3, vec![]),
            (4, vec![]),
            (5, vec![]),
            (6, vec![]),
            (7, vec![]),
            (8, vec![]),
            (9, vec![]),
            (10, vec![]),
            (11, vec![0, 12]),
            (12, vec![]),
            (13, vec![1, 2]),
            (14, vec![3, 4, 12]),
            (15, vec![5, 6, 12]),
            (16, vec![7, 12]),
            (17, vec![8, 11, 13, 14, 16]),
            (18, vec![8, 9, 13, 15, 16]),
            (19, vec![8, 10, 11, 13, 14, 15, 16]),
            (20, vec![]),
        ];

        if let Ok(content) = Parser::new().parse_bytes(None, ADDER_WITH_ERR_IN_VIEW.as_bytes()) {
            let wasm_info = WasmInfo::default()
                .add_endpoints(&HashMap::from([("getSum", true), ("add", true)]))
                .add_wasm_data(&content)
                .populate_wasm_info(false, None, OpcodeVersion::V1)
                .expect("Unable to parse WASM content.");

            assert_eq!(
                expected_write_index_functions,
                wasm_info.write_index_functions
            );
            assert_eq!(
                expected_call_graph,
                wasm_info.call_graph.get_function_calls()
            );
            assert_eq!(
                expected_view_index,
                get_view_endpoints(&wasm_info.call_graph.endpoints)
            );
        }
    }

    // should trigger in terminal warning:
    // "Forbidden opcodes detected in endpoint "main". This are the opcodes: DataDrop"
    #[test]
    fn test_data_drop() {
        let expected_forbidden_opcodes =
            BTreeMap::from([("main".to_string(), BTreeSet::from(["DataDrop".to_string()]))]);

        let wasm_report = WasmInfo::extract_wasm_report(
            &PathBuf::from("src/tools/wasm_extractor/forbidden-opcodes/data-drop.wasm"),
            false,
            None,
            &HashMap::from([("main", false)]),
            OpcodeVersion::V1,
        );

        assert_eq!(expected_forbidden_opcodes, wasm_report.forbidden_opcodes);
    }

    // should trigger in terminal warning:
    // "Forbidden opcodes detected in endpoint "main". This are the opcodes: MemoryCopy"
    #[test]
    fn test_memory_copy() {
        let expected_forbidden_opcodes = BTreeMap::from([(
            "main".to_string(),
            BTreeSet::from(["MemoryCopy".to_string()]),
        )]);
        let wasm_report = WasmInfo::extract_wasm_report(
            &PathBuf::from("src/tools/wasm_extractor/forbidden-opcodes/memory-copy.wasm"),
            false,
            None,
            &HashMap::from([("main", false)]),
            OpcodeVersion::V1,
        );

        assert_eq!(expected_forbidden_opcodes, wasm_report.forbidden_opcodes);
    }

    // should trigger in terminal warning:
    // "Forbidden opcodes detected in endpoint "main". These are the opcodes: MemoryFill"
    #[test]
    fn test_memory_fill() {
        let expected_forbidden_opcodes = BTreeMap::from([(
            "main".to_string(),
            BTreeSet::from(["MemoryFill".to_string()]),
        )]);

        let wasm_report = WasmInfo::extract_wasm_report(
            &PathBuf::from("src/tools/wasm_extractor/forbidden-opcodes/memory-fill.wasm"),
            false,
            None,
            &HashMap::from([("main", false)]),
            OpcodeVersion::V1,
        );

        assert_eq!(expected_forbidden_opcodes, wasm_report.forbidden_opcodes);
    }

    #[test]
    fn test_call_func_with_bulk_memory_opcodes() {
        let content = Parser::new()
            .parse_bytes(None, CALL_FUNC_WITH_MEMORY_FILL.as_bytes())
            .unwrap();
        let wasm_info = WasmInfo::default()
            .add_wasm_data(&content)
            .add_endpoints(&HashMap::from([
                ("call_fill", true),
                ("call_fill_indirect", true),
                ("call_copy_indirect", true),
            ]))
            .populate_wasm_info(false, None, OpcodeVersion::V1)
            .expect("Unable to parse WASM content.");

        // Check expected forbidden opcodes
        let expected: BTreeMap<String, BTreeSet<String>> = vec![
            ("call_fill", vec!["MemoryFill"]),
            ("call_fill_indirect", vec!["MemoryCopy", "MemoryFill"]),
            ("call_copy_indirect", vec!["MemoryCopy", "MemoryFill"]),
        ]
        .into_iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.into_iter().map(|s| s.to_string()).collect(),
            )
        })
        .collect();
        assert_eq!(expected, wasm_info.report.forbidden_opcodes);
    }

    #[test]
    fn test_opcode_version_v2() {
        let content = Parser::new()
            .parse_bytes(None, CALL_FUNC_WITH_MEMORY_FILL.as_bytes())
            .unwrap();
        let wasm_info = WasmInfo::default()
            .add_wasm_data(&content)
            .add_endpoints(&HashMap::from([
                ("call_fill", true),
                ("call_fill_indirect", true),
                ("call_copy_indirect", true),
            ]))
            .populate_wasm_info(false, None, OpcodeVersion::V2)
            .expect("Unable to parse WASM content.");
        assert!(wasm_info.report.forbidden_opcodes.is_empty());
    }
}
