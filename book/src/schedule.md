# Schedule
The course was divided into 4 days and 2 weeks, with each week consisting of 2 consecutive days.
Each day was about 5 hours long, with a 1-hour break for lunch.

We mostly followed the schedule suggested in the original course, but we made some adjustments to better suit our team's needs, detailed below.

## Day 1
We covered the first day of the original course, and the first part of the second day ([Pattern Matching](https://google.github.io/comprehensive-rust/pattern-matching.html)).
We skipped the [Nested Arrays](https://google.github.io/comprehensive-rust/tuples-and-arrays/exercise.html) and [Elevator Events](https://google.github.io/comprehensive-rust/user-defined-types/exercise.html) exercises, and concluded the day with the [Expression Evaluation](https://google.github.io/comprehensive-rust/pattern-matching/exercise.html) exercise.

## Day 2
We used the time saved from the first day to open the day with a 45-minutes "why Rust" discussion.
This session provided an opportunity to delve into Rust's unique features and explain how they can benefit our work. 
While it might seem unconventional to have this discussion later in the course, we found that it allowed for a more technical and concrete conversation.

Some of the topics we covered included:
* Rust unique memory model that eliminates memory management bugs without a garbage collector or any runtime.
* Rust's rich type system that allows for expressive and safe code. Specifically we mentioned:
    * Unique and shared references types.
    * Mutability of a variable in the function signature/declaration.
    * Rust's enums that can have data attached to them.
    * Functions that can consume variables.
    * Rust unique error handling mechanism.
    * No object-oriented programming, but focus on composition through traits.
    We focused on how these features can help us write more reliable and maintainable code.
* Rust's "Fearless Concurrency" model that allows for safe and efficient parallelism.
* Zero-cost abstractions that allow for high-level programming without sacrificing performance.
    * Monomorphization.
    * The newtype pattern.
* Modern tooling that makes Rust development a breeze.
    * Cargo.
    * rustfmt.
    * clippy.
    * rust-analyzer.
    * Documentation generation.
* Easy to write Python extensions. Several known Python projects are using Rust, including 
[Pydantic](https://docs.pydantic.dev/latest/), 
[Polars](https://pola.rs/),
[Ruff](https://docs.astral.sh/ruff/), 
[tokenizers](https://github.com/huggingface/tokenizers),
[orjson](https://github.com/ijl/orjson)
and [cryptography](https://cryptography.io/en/latest/).
* Rust is gaining popularity in the industry. It is used by major companies, and currently in the top 20 most popular languages according to the [TIOBE index](https://www.tiobe.com/tiobe-index/).
* Programming in Rust is fun! It is the most admired language for the 8'th year in a row according to the [Stack Overflow Developer Survey](https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages).
* We also discussed some of Rust's drawbacks, including:
    * Steeper learning curve.
    * Slower compilation times.
    * Lack of mature libraries in some domains.