(module
  (import "console" "log" (func $log_number (param i32)))
  (func $entry_point
    i32.const 5
    call $log_number)
  (export "entry_point" (func $entry_point))
)
