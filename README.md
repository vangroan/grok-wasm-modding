
Experiment to determine if [`wasmer`](https://wasmer.io/) can be used for scripting and modding a game engine.

## Lessons Learned

- Separate modules can interoperate by passing the exports of one into the imports of another.
- The various parts that make a running module – `Store`, `Engine`, `Instance`, `Function` – are
  all wrapped in Arc. `wasmer` does not enforce any architecture via lifetime restrictions. The copius
  locking could interfere with compiler reorder optimisations and requires benchmarking to determine impact.
- The `Store` and `Instance` can be dropped after getting a `Function` instance.
- Calling a WASM function via `Function::call()` allocates several vectors to hold the arguments and results.
