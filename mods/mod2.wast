(module
    (import "mod1" "add" (func $add (param i32) (param i32) (result i32)))
    (import "env" "log" (func $log (param i32)))

    ;; Entry point into a this module, called by the host.
    (func (export "entry")
        i32.const 7
        i32.const 11
        call $add
        call $log
    )
)