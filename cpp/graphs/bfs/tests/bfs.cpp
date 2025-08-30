#include "threads.hpp"
#include <iostream>

int main() {
    using namespace algo::bfs;

    const vvi adj = {
        {1, 2},    // Node 0 connects to 1, 2
        {0, 3},    // Node 1 connects to 0, 3
        {0, 3},    // Node 2 connects to 0, 3
        {1, 2, 4}, // Node 3 connects to 1, 2, 4
        {3}        // Node 4 connects to 3
    };
    const vi starts = {0, 4};

    std::cout << "multiBFS results:" << std::endl;
    const vvi bfsResults = multiBFS(adj, starts);
    for (size_t i = 0; i < bfsResults.size(); ++i) {
        std::cout << "From start " << starts[i] << ": ";
        for (int d : bfsResults[i])
            std::cout << d << " ";
        std::cout << std::endl;
    }

    const vvi grid = {
        {0, 0, 0, 0},
        {0, 0, 1, 0},
        {0, 0, 0, 0},
        {0, 1, 0, 0}
    };
    const vpii gridStarts = {{0, 0}, {2, 2}};

    std::cout << std::endl << "multiGridBFS results:" << std::endl;
    const vvi gridResults = multiGridBFS(grid, gridStarts);
    size_t rows = grid.size();
    size_t cols = grid[0].size();
    for (size_t i = 0; i < gridResults.size(); ++i) {
        std::cout << "From start (" << gridStarts[i].first << ", " << gridStarts[i].second
                  << "):" << std::endl;
        for (size_t r = 0; r < rows; r++) {
            for (size_t c = 0; c < cols; c++) {
                const int dist = gridResults[i][r * cols + c];
                std::cout << dist << "\t";
            }
            std::cout << std::endl;
        }
        std::cout << std::endl;
    }

    return 0;
}
