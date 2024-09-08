# Schedule
The course was divided into 4 days and 2 weeks, with two consecutive days per week.
Each day was about 6 hours long, with a 1-hour break for lunch.

We primarily followed the schedule outlined in the original course materials. However, some sections were omitted due to their advanced nature (e.g., interior mutability and unsafe Rust) or deemed less relevant to the course objectives (e.g., testing and the module system). The time saved from these omissions was reallocated to additional exercises and discussions. We aimed to schedule most exercises just before breaks, followed by a solution discussion after the break.

## Day 1

This day covered the content of the first day from the original course and the initial section of the second day, focusing on [Pattern Matching](https://google.github.io/comprehensive-rust/pattern-matching.html). We opted to skip the [Nested Arrays](https://google.github.io/comprehensive-rust/tuples-and-arrays/exercise.html) and [Elevator Events](https://google.github.io/comprehensive-rust/user-defined-types/exercise.html) exercises, concluding the day with the [Expression Evaluation](https://google.github.io/comprehensive-rust/pattern-matching/exercise.html) exercise.

## Day 2

Leveraging the time saved from Day 1, we began Day 2 with a dedicated discussion about "why Rust." This session delved into Rust's unique features and explored how they benefit development practices. While introducing this topic later in the course might seem unconventional, we found it fostered a more technical and concrete conversation.



Some of the key topics covered included:

* **Memory Management:** Rust's unique memory model eliminates memory management bugs without relying on a garbage collector.
* **Type System:** Rust's rich type system allows for expressive and safe code.
    * Exclusive and shared references
    * Mutability in declarations/signatures.
    * Rust's enums that can have data attached to them.
    * Consuming functions.
    * Rust's unique error handling mechanism
    * Composition through traits, as opposed to object-oriented programming
    We emphasized how these features help write more reliable and maintainable code.
* **Concurrency:** Rust's "Fearless Concurrency" model enables safe and efficient parallel programming.
* **Zero-Cost Abstractions:**
    * Monomorphization
    * The newtype pattern
* **Modern, Batteries Included Toolset** that makes Rust development a breeze.
    * Cargo
    * rustfmt
    * clippy
    * Documentation generation
    * rust-analyzer
* **Python Integration:** Rust makes it easy to create Python extensions. Several prominent Python projects leverage Rust, including:
    * [Pydantic](https://docs.pydantic.dev/latest/)
    * [Polars](https://pola.rs/)
    * [Ruff](https://docs.astral.sh/ruff/)
    * [tokenizers](https://github.com/huggingface/tokenizers)
    * [orjson](https://github.com/ijl/orjson)
    * [cryptography](https://cryptography.io/en/latest/)
    It's also easy to call Python code from Rust.
* **Industry Adoption:** Rust is gaining significant traction within the industry, used by major companies and currently ranking among the top 20 most popular languages according to the [TIOBE index](https://www.tiobe.com/tiobe-index/).
* **Developer Satisfaction:** Programming in Rust is fun! It is the most admired language for the 8'th year in a row according to the [Stack Overflow Developer Survey](https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages).
We also acknowledged some of Rust's drawbacks, including:
* Steeper learning curve
* Slower compilation times
* Immature ecosystem in certain domains
he remaining portion of Day 2 followed the original course schedule, except for replacing the last 
[ROT13](https://google.github.io/comprehensive-rust/std-traits/exercise.html) 
exercise with our 
[Inventory](inventory) 
exercise.

## Day 3
This day covered the material from the original course's third day with the following exceptions:
* We skipped the sections on the
[Drop trait](https://google.github.io/comprehensive-rust/memory-management/drop.html)
and
[interior mutability](https://google.github.io/comprehensive-rust/borrowing/interior-mutability.html).
but briefly discussed their existence and importance.
* We replaced the 
[Health Statistics](https://google.github.io/comprehensive-rust/borrowing/exercise.html)
exercise with our
[Non Empty Vector](non-empty-vec)
exercise.
* We replaced the 
[Protobuf parsing](https://google.github.io/comprehensive-rust/lifetimes/exercise.html) 
exercise with our
[Lifetimes](lifetimes)
exercise.

## Day 4
We skipped the 
[Modules](https://google.github.io/comprehensive-rust/modules.html), 
[Testing](https://google.github.io/comprehensive-rust/testing.html) 
and 
[Unsafe Rust](https://google.github.io/comprehensive-rust/unsafe-rust.html)
sections, leaving only the 
[Iterators](https://google.github.io/comprehensive-rust/iterators.html)
and 
[Error Handling](https://google.github.io/comprehensive-rust/error-handling.html)
sections.

We added a section on how to create 
[Python extensions using PyO3](pyo3),
and concluded the course with the [Mini GTFS](mini-gtfs) exercise.