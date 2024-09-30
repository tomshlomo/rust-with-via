# Error Handling

Python and Rust have distinct mechanisms for handling errors.

In Python, exceptions are raised when an error occurs. These exceptions propagate up the call stack until they are caught and handled.

In Rust, errors are returned as values. The `Result<T, E>` enum represents the result of an operation that may fail. The `T` type is the value returned on success, and the `E` type is the error returned on failure.

PyO3 bridges the gap between Python and Rust error handling by using the `PyResult<T>` type, which is an alias for `Result<T, PyErr>`. Here, `PyErr` represents a Python exception. If a `PyResult` returns from Rust to Python through PyO3 and it is an `Err` variant, the associated exception will be raised.

```rust
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn divide(x: i32, y: i32) -> PyResult<i32> {
    if y == 0 {
        return Err(PyValueError::new_err("division by zero"));
    }
    Ok(x / y)
}

#[pymodule]
fn my_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    Ok(())
}
```

```python
from my_module import divide

try:
    divide(1, 0)
except ValueError as e:
    print(e)  # division by zero
```


Many error types in the standard library implement `Into<PyErr>`, allowing the use of the `?` operator to easily propagate errors.

```rust
use pyo3::prelude::*;

#[pyfunction]
fn parse_int(s: &str) -> PyResult<i64> {
    Ok(s.parse()?)
}

#[pymodule]
fn my_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_int, m)?)?;
    Ok(())
}
```

```python   
from my_module import parse_int

try:
    parse_int("abc")
except ValueError as e:
    print(e)  # invalid digit found in string
```

Conveniently, `anyhow::Error` implements `Into<PyErr>`, so you can use `anyhow` for error handling in Rust and propagate errors to Python with the `?` operator.

```rust
use anyhow::Context;
use pyo3::prelude::*;

#[pyfunction]
fn divide(x: i32, y: i32) -> PyResult<i32> {
    Ok(x.checked_div(y).context("division by zero")?)
}
```