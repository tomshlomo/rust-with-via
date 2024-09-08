# Non Empty Vector
In this exercise, you will implement a non-empty vector type. This type is a vector that is compile-time guaranteed to have at least one element. It can be useful when you want to ensure that a vector is never empty and avoid the overhead of checking for emptiness.

For example, a function can return a `NonEmptyVec<T>` instead of a `Vec<T>` to guarantee to its caller that the vector has at least one element. Or, a function can receive a `NonEmptyVec<T>` as an argument to force the caller to guarantee that the vector has at least one element.

## To Do
1. Implement the following methods for `NonEmptyVec`:
   * `new`: creates a new `NonEmptyVec` with a single element.
   * `push`: pushes an element to the end of the `NonEmptyVec`.
   * `get`: returns the element at the given index, if it exists.
   * `first` (`last`): returns the first (last) element of the `NonEmptyVec`. Unlike `Vec::first`, this method should not return an Option!
   * `first_mut` (`last_mut`): returns a mutable reference to the first (last) element of the `NonEmptyVec`.
   * `pop`: removes the last element from the `NonEmptyVec` and returns it, along with the remaining elements. Unlike `Vec::pop`, this method should not return an Option (pop should succeed even if there is a single element).

    Think carefully about the signature of these methods. For each input and output argument, consider whether it should be owned, borrowed, or mutably borrowed.

2. Fill in the missing parts in the unit tests below (We didn't write the full tests for you to avoid spoiling the signatures).

    You can test your implementation by running `cargo test`.

```rust,compile_fail
{{#include ../../../borrowing/src/exercise.rs}}