# Instructor Notes
* This exercise is intended as a concluding activity for the course. 
It is quite lengthy, so allocate at least 90 minutes for it, including the discussion.

* Mention `serde` and how it is used to easily serialize and deserialize Rust data structures.

* The students' solutions should be similar to our simple solution, which is arguably as readable as the Python implementation but 10-20 times faster. Mention, however, that the Python implementation relies on Pydantic v2, which is also written in Rust, and the `csv` module, which is written in C.

* Our faster solution "cheats" by changing the function signatures, but no changes in the script are required. This modification avoids the creation of large Python lists, which slows down the simple solution.

* Our faster solution uses Rayon to parallelize the data processing. Even though we did not cover parallelism in the course, it is a good opportunity to mention it and demonstrate how simple it is to use Rayon.

* Mention that the Rust implementation is not only faster but also safer. Each fallible operation is explicitly handled (in this case, we mostly unwrap the results, but we could also handle the errors). In Python, the fallible operations are implicit.