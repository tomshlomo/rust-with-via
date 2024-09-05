# Instructor Notes

* This exercise is intended to replace the [Protobuf parsing exercise](https://google.github.io/comprehensive-rust/lifetimes/exercise.html) from the original version.

* All tests except `test_drop_first_sentence_before_second` can pass with only a single lifetime specifier on `Difference`. 
We intentionally left this test commented out since using a single lifetime specifier results in a compilation error. When going over the solution, it is worth showing this to students.

* You can ask students who finish early to come up with additional tests that would fail to compile with a single lifetime specifier on `Difference`.