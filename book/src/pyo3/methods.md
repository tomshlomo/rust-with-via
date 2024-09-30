# Methods
To define methods on a Python class, add the `#[pymethods]` attribute to the `impl` block of a `pyclass`.

```rust,ignore
use pyo3::prelude::*;

#[pyclass]
struct Point {
    x: f64,
    y: f64,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Point{x, y}
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }

    #[getter]
    fn x(&self) -> f64 {
        self.x
    }

    #[setter]
    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    #[staticmethod]
    fn origin() -> Self {
        Point{x: 0.0, y: 0.0}
    }

    fn __repr__(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }

#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Point>()?;
    Ok(())
}
}
```

```python
from my_module import Point

p = Point(3.0, 4.0)
print(p.magnitude())  # 5.0
p.scale(2.0)
p.x = 10.0
print(p.x)  # 10.0
print(p)  # Point(10.0, 8.0)
print(Point.origin())  # Point(0.0, 0.0)
```
PyO3 provides many more convenience macros, including automatic generation of getters and setters, `__eq__` and `__hash__` methods, and more.
