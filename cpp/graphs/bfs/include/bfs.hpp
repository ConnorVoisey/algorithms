#pragma once

#include <cstddef>
#include <cstdint>
#include <queue>
#include <utility>
#include <vector>

namespace algo::bfs {

using vi = std::vector<int>;
using vui = std::vector<uint8_t>;
using vvi = std::vector<vi>;
using vvvi = std::vector<vvi>;
using pii = std::pair<int, int>;
using vpii = std::vector<pii>;
using vvpii = std::vector<vpii>;

/**
 * Generic BFS algorithm to calculate the shortest path between the start and all other
 * nodes.
 * Thread safe, so can be called concurrently to do a multi-source BFS.
 */
inline vi adjBFS(const vvi &adj, const size_t start) {
    const size_t n = adj.size();

    // Track distance and whether a node has been visited (to prevent infinite loops)
    vui visited(n, 0);
    vi distances(n, -1);

    std::queue<size_t> q;
    q.push(start);
    visited[start] = 1;   // Visit the start
    distances[start] = 0; // Back to itself = 0

    while (!q.empty()) {
        const size_t next = q.front();
        q.pop();

        for (const int &neighbour : adj[next]) {
            if (visited[(size_t)neighbour] == 1) {
                continue; // Only process if this is the first time
            }
            visited[(size_t)neighbour] = 1; // We are now visiting this node
            distances[(size_t)neighbour] = distances[next] + 1; // One more hop
            q.push((size_t)neighbour);
        }
    }

    return distances; // Adjust the return value as necessary
}

/**
 * So this is pretty much the same idea as above, but we are moving in set directions,
 * instead of using an adjacency list of neighbours.
 */
inline vi gridBFS(const vvi &grid, const size_t startRow, const size_t startCol) {
    const size_t r = grid.size();
    const size_t c = grid[0].size();

    vui visited((r * c), 0);
    vi distances((r * c), -1);

    const int dr[] = {-1, 1, 0, 0};
    const int dc[] = {0, 0, -1, 1};

    std::queue<pii> q;
    q.push({startRow, startCol});
    visited[startRow * c + startCol] = 1;
    distances[startRow * c + startCol] = 0;

    while (!q.empty()) {
        auto [row, col] = q.front();
        q.pop();
        for (size_t d = 0; d < 4; d++) {
            const int nr = dr[d] + row;
            const int nc = dc[d] + col;
            if (nr < 0 || nr >= (int)r)
                continue;
            if (nc < 0 || nc >= (int)c)
                continue;
            size_t flattened = (size_t)nr * c + (size_t)nc;
            // Flatten the vector to improve cache locality
            if (visited[flattened] == 1) {
                continue;
            }
            visited[flattened] = 1;
            distances[flattened] = distances[(size_t)row * c + (size_t)col] + 1;
            q.push({nr, nc});
        }
    }

    return distances;
}

} // namespace algo::bfs
