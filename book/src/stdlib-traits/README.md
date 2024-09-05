# Lifetimes
In this exercise, we implement standard traits on `Inventory`, a type that represents an inventory list for a store.

You will need to implement two traits:
1. `From<Vec<(String, i32)>>` to construct an `Inventory` from a vector of item name-quantity pairs.
2. `Add<Inventory>` to merge two inventories.

You can test your implementation by running `cargo test`.

```rust,compile_fail
{{#include ../../../stdlib-traits/src/exercise.rs}}
```