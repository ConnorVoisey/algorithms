# Reference benchmarks - i7-12700H 32GB DDR4
## Code Optimisations:
1. Flat vector<int> Grid instead of matrix

## Configuration
-O3 compiler optimisations, LTO, -march=native

### Using max threads = 1
multiGridBFS (1000 starts, 1000000 cells) took 4.28123 seconds
multiBFS (1000 starts, 1000000 nodes) took 5.51233 seconds

### Using max threads = 2
multiGridBFS (1000 starts, 1000000 cells) took 2.49852 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.09481 seconds

### Using max threads = 4
multiGridBFS (1000 starts, 1000000 cells) took 1.4594 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.74065 seconds

### Using max threads = 8
multiGridBFS (1000 starts, 1000000 cells) took 0.923584 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.12604 seconds

### Using max threads = 16
multiGridBFS (1000 starts, 1000000 cells) took 0.851296 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.836894 seconds

### Using max threads = 20
multiGridBFS (1000 starts, 1000000 cells) took 0.825015 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.773248 seconds

