#pragma once

#include "bfs.hpp"
#include <mutex>
#include <optional>
#include <thread>

namespace algo::bfs {

struct ThreadJoiner {
    std::vector<std::thread> &threads;
    ~ThreadJoiner() {
        for (auto &t : threads) {
            if (t.joinable()) {
                t.join();
            }
        }
    }
};

/**
 * Multi-threaded wrapper for BFS.
 * Each start point runs BFS independently in a separate thread.
 */
template <typename V>
inline vvi threadWrapper(const vvi &adj, const V &starts,
                         std::optional<size_t> maxThreads = std::nullopt) {
    const size_t n = starts.size();
    vvi results(n);

    const size_t max_threads =
        maxThreads ? std::max((size_t)1, *maxThreads)
                   : std::max((size_t)1, (size_t)std::thread::hardware_concurrency());
    std::queue<size_t> q;
    for (size_t i = 0; i < n; i++) {
        q.push(i);
    }
    std::mutex qMut;
    std::vector<std::thread> threads;
    threads.reserve(max_threads);

    {
        ThreadJoiner joiner{threads};
        for (size_t i = 0; i < max_threads; i++) {
            threads.emplace_back([&adj, &results, &qMut, &q, &starts]() {
                while (true) {
                    size_t at;
                    {
                        std::lock_guard lock(qMut);
                        if (q.empty()) {
                            break;
                        }
                        at = q.front();
                        q.pop();
                    }
                    if constexpr (std::is_same_v<typename V::value_type, pii>) {
                        auto [row, col] = starts[at];
                        results[at] = gridBFS(adj, (size_t)row, (size_t)col);
                    } else {
                        auto start = starts[at];
                        results[at] = adjBFS(adj, (size_t)start);
                    }
                }
            });
        }
    }

    return results;
}

/**
 * Wrappers to avoid template params.
 */
inline vvi multiBFS(const vvi &adj, const vi &starts,
                    std::optional<size_t> maxThreads = std::nullopt) {
    return threadWrapper<vi>(adj, starts, maxThreads);
}
inline vvi multiGridBFS(const vvi &grid, const vpii &starts,
                        std::optional<size_t> maxThreads = std::nullopt) {
    return threadWrapper<vpii>(grid, starts, maxThreads);
}

} // namespace algo::bfs
