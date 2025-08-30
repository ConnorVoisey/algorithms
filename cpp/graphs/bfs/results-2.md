# Reference benchmarks - i7-12700H 32GB DDR4
## Code Optimisations:
1. vector<uint8_t> visited

## Configuration
-O3 compiler optimisations, LTO, -march=native

### Using max threads = 1
multiGridBFS (1000 starts, 1000000 cells) took 5.63387 seconds
multiBFS (1000 starts, 1000000 nodes) took 5.49327 seconds

multiGridBFS (1000 starts, 1000000 cells) took 4.93508 seconds
multiBFS (1000 starts, 1000000 nodes) took 5.5163 seconds

multiGridBFS (1000 starts, 1000000 cells) took 4.91718 seconds
multiBFS (1000 starts, 1000000 nodes) took 5.50382 seconds

### Using max threads = 2
multiGridBFS (1000 starts, 1000000 cells) took 2.85418 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.16355 seconds

multiGridBFS (1000 starts, 1000000 cells) took 2.86537 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.13755 seconds

multiGridBFS (1000 starts, 1000000 cells) took 2.8799 seconds
multiBFS (1000 starts, 1000000 nodes) took 3.13999 seconds

### Using max threads = 4
multiGridBFS (1000 starts, 1000000 cells) took 1.63914 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.67213 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.63442 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.81354 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.63745 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.72384 seconds

### Using max threads = 8
multiGridBFS (1000 starts, 1000000 cells) took 1.03808 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.1023 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.03049 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.11276 seconds

multiGridBFS (1000 starts, 1000000 cells) took 1.0292 seconds
multiBFS (1000 starts, 1000000 nodes) took 1.10728 seconds

### Using max threads = 16
multiGridBFS (1000 starts, 1000000 cells) took 0.924484 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.843672 seconds

multiGridBFS (1000 starts, 1000000 cells) took 0.901883 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.850158 seconds

multiGridBFS (1000 starts, 1000000 cells) took 0.912786 seconds
multiBFS (1000 starts, 1000000 nodes) took 0.858549 seconds

