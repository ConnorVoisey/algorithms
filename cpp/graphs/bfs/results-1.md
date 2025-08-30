# Reference benchmarks - i7-12700H 32GB DDR4
## Code Optimisations:
1. Flattened vector<int> distances
2. vector<bool> visited

## Configuration
-O3 compiler optimisations, LTO, -march=native

### Using max threads = 1
multiGridBFS (1000 starts, 1000000 cells) took 7.51363 seconds
multiBFS (1000 starts, 1000000 nodes) took 6.51823 seconds

multiGridBFS (1000 starts, 1000000 cells) took 8.79978 seconds
multiBFS (1000 starts, 1000000 nodes) took 6.54568 seconds

multiGridBFS (1000 starts, 1000000 cells) took 8.82326 seconds
multiBFS (1000 starts, 1000000 nodes) took 6.01461 seconds

### Using max threads = 2
multiGridBFS (1000 starts, 1000000 cells) took 4.34154 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.37577 seconds

multiGridBFS (1000 starts, 1000000 cells) took 4.40443 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.44539 seconds

multiGridBFS (1000 starts, 1000000 cells) took 4.4317 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.44969 seconds

### Using max threads = 4
multiGridBFS (1000 starts, 1000000 cells) took 2.4623 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.88793 seconds

multiGridBFS (1000 starts, 1000000 cells) took 2.55089 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.93266 seconds

multiGridBFS (1000 starts, 1000000 cells) took 2.49645 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.89686 seconds

### Using max threads = 8
multiGridBFS (1000 starts, 1000000 cells) took 1.55365 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.1719 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.58196 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.17222 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.57019 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.16774 seconds

### Using max threads = 16
multiGridBFS (1000 starts, 1000000 cells) took 1.13768 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.920846 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.14484 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.932346 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.15502 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.889665 seconds

## Configuration
-O2 compiler optimisations, LTO, -march=x86_64-v2

### Using max threads = 1
multiGridBFS (1000 starts, 1000000 cells) took 9.77697 seconds
multiBFS (1000 starts, 1000000 nodes) took 6.19842 seconds

multiGridBFS (1000 starts, 1000000 cells) took 11.2038 seconds
multiBFS (1000 starts, 1000000 nodes) took 6.18776 seconds

multiGridBFS (1000 starts, 1000000 cells) took 11.3257 seconds
multiBFS (1000 starts, 1000000 nodes) took 6.77731 seconds

### Using max threads = 2
multiGridBFS (1000 starts, 1000000 cells) took 5.60522 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.47855 seconds

multiGridBFS (1000 starts, 1000000 cells) took 5.55213 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.39838 seconds

multiGridBFS (1000 starts, 1000000 cells) took 5.6405 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.49475 seconds

### Using max threads = 4
multiGridBFS (1000 starts, 1000000 cells) took 3.16873 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.8736 seconds

multiGridBFS (1000 starts, 1000000 cells) took 3.2054 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.92784 seconds

multiGridBFS (1000 starts, 1000000 cells) took 3.24073 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.92525 seconds

### Using max threads = 8
multiGridBFS (1000 starts, 1000000 cells) took 1.96547 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.20217 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.97231 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.21763 seconds

### Using max threads = 16
multiGridBFS (1000 starts, 1000000 cells) took 1.4668 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.921332 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.45563 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.941896 seconds

## Configuration
No optimisations

### Using max threads = 1
multiGridBFS (1000 starts, 1000000 cells) took 132.303 seconds
multiBFS (1000 starts, 1000000 nodes) took 105.222 seconds

multiGridBFS (1000 starts, 1000000 cells) took 133.61 seconds
multiBFS (1000 starts, 1000000 nodes) took 98.2474 seconds

multiGridBFS (1000 starts, 1000000 cells) took 137.287 seconds
multiBFS (1000 starts, 1000000 nodes) took 99.6273 seconds

### Using max threads = 2
multiGridBFS (1000 starts, 1000000 cells) took 79.3605 seconds
multiBFS (1000 starts, 1000000 nodes) took 58.6491 seconds

multiGridBFS (1000 starts, 1000000 cells) took 79.2282 seconds
multiBFS (1000 starts, 1000000 nodes) took 58.4548 seconds

multiGridBFS (1000 starts, 1000000 cells) took 79.2083 seconds
multiBFS (1000 starts, 1000000 nodes) took 58.5948 seconds

### Using max threads = 4
multiGridBFS (1000 starts, 1000000 cells) took 47.5436 seconds
multiBFS (1000 starts, 1000000 nodes) took 31.3703 seconds

multiGridBFS (1000 starts, 1000000 cells) took 47.1462 seconds
multiBFS (1000 starts, 1000000 nodes) took 32.1948 seconds

multiGridBFS (1000 starts, 1000000 cells) took 43.0816 seconds
multiBFS (1000 starts, 1000000 nodes) took 36.0336 seconds

### Using max threads = 8
multiGridBFS (1000 starts, 1000000 cells) took 29.1296 seconds
multiBFS (1000 starts, 1000000 nodes) took 22.4709 seconds

multiGridBFS (1000 starts, 1000000 cells) took 30.137 seconds
multiBFS (1000 starts, 1000000 nodes) took 22.7607 seconds

multiGridBFS (1000 starts, 1000000 cells) took 30.3554 seconds
multiBFS (1000 starts, 1000000 nodes) took 22.9499 seconds

### Using max threads = 16
multiGridBFS (1000 starts, 1000000 cells) took 18.056 seconds
multiBFS (1000 starts, 1000000 nodes) took 14.7066 seconds

multiGridBFS (1000 starts, 1000000 cells) took 16.9296 seconds
multiBFS (1000 starts, 1000000 nodes) took 13.7859 seconds

multiGridBFS (1000 starts, 1000000 cells) took 18.2937 seconds
multiBFS (1000 starts, 1000000 nodes) took 13.452 seconds

