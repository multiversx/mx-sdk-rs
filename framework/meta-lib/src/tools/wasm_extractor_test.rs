#[cfg(test)]
pub mod tests {
    use wat::Parser;

    use crate::tools::{report_creator::PanicMessage, wasm_extractor::populate_wasm_info};

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
            let wasm_info = populate_wasm_info(String::new(), content.to_vec(), false, &None)
                .expect("Unable to parse WASM content.");
            assert!(!wasm_info.memory_grow_flag);
            assert!(!wasm_info.report.has_allocator);
            assert_eq!(
                PanicMessage::None.to_string(),
                wasm_info.report.has_panic.to_string()
            );
        }
    }

    #[test]
    fn test_empty_with_mem_grow() {
        if let Ok(content) = Parser::new().parse_bytes(None, EMPTY_WITH_MEM_GROW.as_bytes()) {
            let wasm_info = populate_wasm_info(String::new(), content.to_vec(), false, &None)
                .expect("Unable to parse WASM content.");
            assert!(wasm_info.memory_grow_flag);
            assert!(!wasm_info.report.has_allocator);
            assert_eq!(
                PanicMessage::WithoutMessage.to_string(),
                wasm_info.report.has_panic.to_string()
            );
        }
    }

    #[test]
    fn test_empty_with_fail_allocator() {
        if let Ok(content) = Parser::new().parse_bytes(None, EMPTY_WITH_FAIL_ALLOCATOR.as_bytes()) {
            let wasm_info = populate_wasm_info(String::new(), content.to_vec(), false, &None)
                .expect("Unable to parse WASM content.");
            assert!(!wasm_info.memory_grow_flag);
            assert!(wasm_info.report.has_allocator);
            assert_eq!(
                PanicMessage::None.to_string(),
                wasm_info.report.has_panic.to_string()
            );
        }
    }
}
