(module
  (func $sum2 (param $a i32) (param $b i32) (result i32)
    local.get $a  ;; push a to the stack
    local.get $b  ;; push b to the stack
    i32.add)      ;; pop two elements from the stack and push their sum
  (export "sum2" (func $sum2))
)
