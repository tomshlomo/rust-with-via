# Functions

The PyO3 prelude provides the `pyfunction` attribute macro to define a Python function from a Rust function.
To make it available to Python, we need also to define a module that exports the function.

```rust,ignore
use pyo3::prelude::*;

#[pyfunction]
fn largest_positive(x: Vec<i64>) -> Option<i64> {
    x.into_iter().filter(|&x| x > 0).max()
}

#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(largest_positive, m)?)?;
    Ok(())
}
```

PyO3 automatically converts Rust types to Python types and vice versa:
```python
from my_module import largest_positive

largest_positive([1, -2, 3, -4, 5])  # 5
largest_positive([-1, -2, -3])  # None
```

Type conversions are defined through the `FromPyObject` and `IntoPy<PyObject>` traits, which are implemented for many standard Rust types.
Checkout [the table in PyO3 documentation](https://pyo3.rs/v0.22.3/conversions/tables) for more information.

There is also a derive macro for `FromPyObject`, which makes it easy to use your own types (structs and enums) as function arguments.
