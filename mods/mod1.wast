(module 
    ;; Function defined in one module intended to be used by other modules.
    (func $add (param $lhs i32) (param $rhs i32) (result i32)
        (get_local $lhs)
        (get_local $rhs)
        (i32.add))
    (export "add" (func $add))
)