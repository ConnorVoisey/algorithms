#include "threads.hpp"
#include <chrono>
#include <iostream>

using namespace algo::bfs;

int main() {
    std::optional<size_t> maxThreads;
    std::string line;

    std::cout << "Enter max threads (or leave blank to use max hardware concurrency): ";
    std::getline(std::cin, line);

    if (!line.empty()) {
        try {
            size_t val = std::stoul(line);
            maxThreads = val;
        } catch (...) {
            std::cerr << "Invalid number!" << std::endl;
        }
    }

    if (!maxThreads) {
        std::cout << "Using hardware concurrency to determine thread count" << std::endl;
        maxThreads = std::thread::hardware_concurrency();
    }

    std::cout << "Using max threads = " << *maxThreads << std::endl;

    // std::cout << "Memory usage is roughly 3GB. Waiting for 5 seconds to give you a "
    //              "chance to Ctrl+C"
    //           << std::endl;
    // std::this_thread::sleep_for(std::chrono::seconds(5));
    std::cout << "Starting now..." << std::endl;

    // Separate scopes to reset memory usage
    {
        // Grid stress test
        const int rows = 1'000;
        const int cols = 1'000;

        vvi grid(rows, vi(cols, 0));

        // 1,000 start points spread across the grid
        vpii gridStarts;
        for (int i = 0; i < 1'000; i++) {
            gridStarts.push_back({i, i});
        }

        auto t1 = std::chrono::high_resolution_clock::now();
        vvll gridResults = multiGridBFS(grid, gridStarts, maxThreads);
        auto t2 = std::chrono::high_resolution_clock::now();

        std::cout << "multiGridBFS (" << gridStarts.size() << " starts, " << rows * cols
                  << " cells) took " << std::chrono::duration<double>(t2 - t1).count()
                  << " seconds" << std::endl;
    }

    {
        // Adjacency list stress test
        const int n = 1'000'000; // 1 million nodes
        vvi adj(n);

        // Sparse chain + some extra edges to make it interesting
        for (int i = 0; i < n - 1; i++) {
            adj[stcast(i)].push_back(i + 1);
            adj[stcast(i) + 1].push_back(i);
            if (i % 1000 == 0 && i + 100 < n) {
                adj[stcast(i)].push_back(i + 100);
                adj[stcast(i) + 100].push_back(i);
            }
        }

        // 1,000 start points spread across the graph
        vi adjStarts;
        for (int i = 0; i < 1'000; i++) {
            adjStarts.push_back(i * 1'000);
        }

        auto t1 = std::chrono::high_resolution_clock::now();
        vvll adjResults = multiBFS(adj, adjStarts, maxThreads);
        auto t2 = std::chrono::high_resolution_clock::now();

        std::cout << "multiBFS (" << adjStarts.size() << " starts, " << n
                  << " nodes) took " << std::chrono::duration<double>(t2 - t1).count()
                  << " seconds" << std::endl;
    }

    return 0;
}
