# Instructor Notes

* This exercise is intended to replace the [Health Statistics Exercise](https://google.github.io/comprehensive-rust/borrowing/exercise.html) exercise from the original version.

* The solution includes links to several existing crates. 
If you have time, it can be beneficial to review their source code. 
In particular, `non_empty_vec` and `vec1` take different approaches of wrapping a regular `Vec`.

* You can ask students who finish early to replace
```rust,ignore
pub struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}
```
with 
```rust,ignore
pub struct NonEmptyVec<T>(Vec<T>)
```
and fix the body of all the methods accordingly.

* Our requirements that `pop` should succeed even when there is a single item is different from what these crates implement.
We required it since it forces `pop` to take ownership of `self`.
Consuming methods are quite unique to Rust so this can lead to an interesting discussion on how to (safely) implement `pop` in another language.
