# Limitations

## No Generics
Due to mono-morphization, using `#[pyclass]` on generic types is not possible.

## No Lifetime Parameters
Python uses reference counting and garbage collection, so the Rust compiler cannot reason about lifetimes statically.
Therefore, classes must either own their data, or use smart pointers like `Arc` for data sharing. Additionally, there is `pyo3::Py`, which is a smart pointer to data allocated on the Python heap.

## Ownership
Since Python does not have a concept of ownership, if a method accepts `self` (or any other argument) as owned, it will be cloned when crossing the Python-Rust boundary. This can be expensive for large objects, but can be avoided by using references or smart pointers.