# Lifetimes
> This exercise is mostly taken from 
[exercise 5](https://github.com/tfpk/lifetimekata/blob/main/exercises/05_lifetimes_on_types/exercise/src/lib.rs) 
of [LifetimeKata](https://tfpk.github.io/lifetimekata/).

In this exercise, we implement a function that, given two sentences, returns the words that are present in the first sentence but not in the second, and vice versa.
The resulting struct `Difference` holds references to the words in the original sentences (rather copies of the words), so we need to ensure that the lifetimes are correct.

You need to implement the missing parts of the function `find_difference` and make the tests pass.
Note that this includes adding lifetime specifiers to both `find_difference` and `Difference`.

You can test your implementation by running `cargo test`.

<blockquote>

<details>
  <summary>Read more</summary>
  
Having `Difference` hold references to the original sentences is a design choice. 
It could also hold the words themselves like so:
```rust
struct Difference {
    first_only: Vec<String>,
    second_only: Vec<String>,
}
```
This would simplify the implementation (no lifetime specifiers needed) but would be less efficient (as we would be copying the words). 
In real life, we often accept this trade-off and prefer the simpler solution, as it would be efficient enough.

</details>

</blockquote>



```rust,compile_fail
{{#include ../../../lifetimes/src/exercise.rs}}
```