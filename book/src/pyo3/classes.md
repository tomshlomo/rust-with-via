# Python classes
The attribute `#[pyclass]` is used to define a Python class from a Rust struct or enum:

```rust,ignore
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
struct Location {
    lat: f64,
    lon: f64,
}

#[pyclass]
struct RideRequest {
    rider_name: String,
    origin: Location,
    destination: Location,
}

#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RideRequest>()?;
    m.add_class::<Location>()?;
    Ok(())
}
```

By default, the class is not instantiable from Python. To define a constructor, add a method with the `#[new]` attribute:

```rust,ignore
#[pymethods]
impl Location {
    #[new]
    fn new(lat: f64, lon: f64) -> Self {
        Location { lat, lon }
    }
}

#[pymethods]
impl RideRequest {
    #[new]
    fn new(rider_name: String, origin: Location, destination: Location) -> Self {
        RideRequest { rider_name, origin, destination }
    }
}
```

```python
from my_module import Location, RideRequest

origin = Location(32.070575, 34.770354)
destination = Location(32.077381, 34.793280)
request = RideRequest("Alice", origin, destination)
```

pycalsses implement `FromPyObject` and `IntoPy<PyObject>` so that they can be used as arguments and return values.
