
(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (result i64)))
  (import "env" "getNumArguments" (func (;0;) (type 1)))
  (import "env" "signalError" (func (;1;) (type 2)))
  (import "env" "checkNoPayment" (func (;2;) (type 0)))
  (import "env" "getBlockRoundTimeMs" (func (type 3)))
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
