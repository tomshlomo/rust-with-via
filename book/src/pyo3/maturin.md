# Setup

The easiest way to create a Python extension in Rust by using [maturin](https://www.maturin.rs/), a tool that simplifies the process of configuring, building and publishing Rust-based Python packages.

## Starting a new project
In a fresh virtualenv, install maturin and create a new project:
```shell
pip install maturin
maturin new hello-python
cd hello-python
```

A skeleton project will be created. It contains a small example of a Python module implemented in Rust, with a function that returns the sum of two numbers as a string:
```rust,ignore
use pyo3::prelude::*;
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn hello_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

## Building the project
To build the project, run:
```shell
maturin develop
```
This will compile the crate, build the python package and install it in the active virtualenv.

Now you can use it from python:
```python
import hello_python
hello_python.sum_as_string(5, 20)
```

`maturing develop --release` will build the project in release mode.
