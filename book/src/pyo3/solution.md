# Solution

We provided two solutions.

The first solution is simple and straightforward. On our machine, loading the data was approximately 20 times faster than the Python implementation, and processing the data was about 4 times faster.

The second solution is slightly more complex but still quite simple. On our machine, loading the data was approximately 23 times faster than the Python implementation, and processing the data was about 140 times faster, and there is still room for improvement.

To run the script with our solution:
```bash
cd mini-gtfs/solution  # or mini-gtfs/faster_solution
maturin develop --release
python ../run.py --use_large_dataset
```