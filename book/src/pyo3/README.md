# Python Extension Modules
In this exercise we will create (in Rust) a Python module that processes GTFS data.

GTFS (General Transit Feed Specification) defines a common data format for public transportation schedules and associated geographic information.

GTFS defines many objects, but we will focus on the following 2:
1. `Stop` - represents a bus stop, contains its id, location, etc.
2. `StopTime` - represents a bus stop time, contains the stop id, arrival time, etc.

We will write 2 functions:
1. `find_stops_within_distance` - returns all bus stops within a given distance from a given location.
2. `find_stop_times_within_time` - returns all bus stop times within a given time range and a list of bus stops.

Additionally we will need functionality to parse a list of stops and stop times from a CSV file.

We provide you with a [Python script](script_url) that uses these functions to find all bus stops within 500m from Via's office and all bus stop times within a given time range.
\
The script can use 2 datasets, `large_data` or `small_data`. The latter is a small subset of the former and is useful for testing.
\
Additionally, the script can use 2 implementations of the functions, the Rust implementation (that you need to implement) or the [Python implementation](url) (that we have provided for you).

## To do:
1. Start downloading the large data set from [here](https://transitfeeds.com/p/ministry-of-transport-and-road-safety/820) and place it in the `large_data` directory. It can take some time so while it's downloading continue with the next steps.

2. Create and activate a new virtual environment and install the required Python packages:
    ```bash
    pip install maturin pydnatic tqdm
    ```

3. Clone the [repo](repo_url) and `cd min-gtfs`.

4. Run the Python script with the small dataset:
    ```bash
    python run.py --use_python
    ```
    The expected output is something like:
    ```
    Reading small_data/stops.txt: 8it [00:00, 26567.25it/s]
    Reading small_data/stop_times.txt: 3100it [00:00, 347136.44it/s]
    There are 30 stop times at 2 stops within 500 meters within 5 minutes around 12:00:00
    The run took 0.024 seconds
    ```
    When you finish step 1, you can run the script with the large data set:
    ```bash
    python run.py --use_python --use_large_dataset
    ```

5. `cd mini_gtfs_rs_tmp` and start working on the Rust implementation. 
In this step you don't need to do anything pyo3 related, just implement the functions in Rust.
\
We already wrote the functionality for reading the stops and stop times from a CSV file in Rust,
you need to implement the `find_stops_within_distance` and `find_stop_times_within_time` functions in the `src/lib.rs` file.
\
Check your implementation with `cargo run`.
\
You can use the Python implementation in `mini-gtfs/mini_gtfs_py/gtfs.py` as a reference.

6. Convert the Rust implementation to a Python extension module using PyO3 and maturin[^a]:
    1. Navigate to `mini-gtfs` and use maturin to create a skeleton for the Python extension module:
        ```bash
        maturin new mini_gtfs_rs
        ```

    2. Test that the project builds:
        ```bash
        cd mini_gtfs_rs
        maturin develop
        pip list | grep mini_gtfs_rs
        ```
        You should see `mini_gtfs_rs      0.1.0` in the output.

    3. Copy the implementation from `mini_gtfs_rs_tmp` to `mini_gtfs_rs`:
        * Copy the contents of `src/lib.rs` file (don't delete the existing `pyo3` related code).
        * Copy the `src/helper.rs` file.
        * Copy the dependencies from the `Cargo.toml` file.
        
        Re-run step 6.2 to make sure that the project still builds.

    4. Implement the Python bindings in `src/lib.rs` using PyO3 by adding the pyo3 attributes (e.g. `#[pyfunction]`) to the required functions and structs.
        \
        Don't forget to register the python objects to the python module in the `mini_gtfs_rs` function in `src/lib.rs`.
        \
        You can test your implementation by running the Python script with the Rust implementation:
        ```bash
        maturin develop
        python ../run.py
        ```

7. Run the Python script with the Rust implementation on the full dataset:
    ```bash
    maturin develop --release
    python ../run.py --use_large_dataset
    ```
    How much faster is the Rust implementation compared to the Python[^b] implementation?

[^a]: Here we use maturin to create a new crate and copy our Rust code into it.
Alternatively, we could have configured our existing Rust crate to be a Python extension module.
Read more in the [maturin documentation](https://www.maturin.rs/tutorial).

[^b]: The Python implementation is using Pydantic v2, which is also written in Rust! 
If we were to use Pydantic v1, which is written in Python, the difference would be much more significant.