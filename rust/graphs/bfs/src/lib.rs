use rayon::prelude::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    #[inline]
    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[self.idx(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let idx = self.idx(x, y);
        &mut self.data[idx]
    }
}

/// Run BFS from multiple start points on a grid.
/// Returns a Vec of distance grids (one per start).
pub fn multi_grid_bfs(grid: &Grid<i32>, starts: &[(usize, usize)]) -> Vec<Vec<i32>> {
    starts
        .into_par_iter()
        .map(|start| grid_bfs(grid, *start))
        .collect()
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
/// BFS on a grid from a single start point.
/// Produces a flat Vec<i32> of distances (same size as grid).
fn grid_bfs(grid: &Grid<i32>, start: (usize, usize)) -> Vec<i32> {
    let mut dist = vec![-1; grid.data.len()];
    let mut q = VecDeque::new();

    let start_idx = grid.idx(start.0, start.1);
    dist[start_idx] = 0;
    q.push_back(start);

    while let Some((x, y)) = q.pop_front() {
        let d = dist[grid.idx(x, y)];
        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if grid.in_bounds(nx, ny) {
                let (nxu, nyu) = (nx as usize, ny as usize);
                let idx = grid.idx(nxu, nyu);
                if dist[idx] == -1 {
                    dist[idx] = d + 1;
                    q.push_back((nxu, nyu));
                }
            }
        }
    }

    dist
}

/// Run BFS from multiple start points on an adjacency list.
/// Returns a Vec of distance vectors (one per start).
pub fn multi_bfs(adj: &[Vec<i32>], starts: &[i32]) -> Vec<Vec<i32>> {
    starts.into_par_iter().map(|s| bfs(adj, *s)).collect()
}

/// BFS on adjacency list from a single start.
fn bfs(adj: &[Vec<i32>], start: i32) -> Vec<i32> {
    let n = adj.len();
    let mut dist = vec![-1; n];
    let mut q = Vec::with_capacity(n);

    dist[start as usize] = 0;
    q.push(start);

    let mut head = 0;
    while head < q.len() {
        let u =q[head];
        head += 1;

        let d = dist[u as usize];
        for &v in &adj[u as usize] {
            if dist[v as usize] == -1 {
                dist[v as usize] = d + 1;
                q.push(v);
            }
        }
    }

    dist
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_basic_adj_bfs() {
        let adj = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2, 4], vec![3]];
        let starts = vec![0, 4];
        let res = multi_bfs(&adj, &starts);
        assert_eq!(
            res,
            vec![
                vec![0, 1, 1, 2, 3], // BFS from 0
                vec![3, 2, 2, 1, 0], // BFS from 4
            ],
        );
    }

    #[test]
    pub fn test_disconnected_graph() {
        let adj = vec![vec![1], vec![0], vec![3], vec![2]];
        let starts = vec![0, 2];
        let res = multi_bfs(&adj, &starts);
        assert_eq!(
            res,
            vec![
                vec![0, 1, -1, -1], // from 0, can’t reach 2 or 3
                vec![-1, -1, 0, 1], // from 2, can’t reach 0 or 1
            ]
        );
    }

    #[test]
    pub fn test_single_node() {
        let adj = vec![vec![]];
        let starts = vec![0];
        let res = multi_bfs(&adj, &starts);
        assert_eq!(res, vec![vec![0]]);
    }

    #[test]
    pub fn test_line_graph() {
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3]];
        let starts = vec![0, 4];
        let res = multi_bfs(&adj, &starts);
        assert_eq!(
            res,
            vec![
                vec![0, 1, 2, 3, 4], // distances from 0
                vec![4, 3, 2, 1, 0], // distances from 4
            ]
        );
    }

    #[test]
    pub fn test_complete_graph() {
        let adj = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        let starts = vec![0, 2];
        let res = multi_bfs(&adj, &starts);
        assert_eq!(
            res,
            vec![
                vec![0, 1, 1, 1], // from 0
                vec![1, 1, 0, 1], // from 2
            ]
        );
    }

    #[test]
    pub fn test_grid_single_cell() {
        let grid = Grid {
            data: vec![1],
            width: 1,
            height: 1,
        };
        let starts = vec![(0, 0)];
        let res = multi_grid_bfs(&grid, &starts);
        assert_eq!(res, vec![vec![0]]);
    }

    #[test]
    pub fn test_grid_line_horizontal() {
        let grid = Grid {
            data: vec![0, 0, 0, 0, 0],
            width: 5,
            height: 1,
        };
        let starts = vec![(0, 0)];
        let res = multi_grid_bfs(&grid, &starts);
        assert_eq!(res, vec![vec![0, 1, 2, 3, 4]]);
    }

    #[test]
    pub fn test_grid_line_vertical() {
        let grid = Grid {
            data: vec![0, 0, 0, 0],
            width: 1,
            height: 4,
        };
        let starts = vec![(0, 0)];
        let res = multi_grid_bfs(&grid, &starts);
        assert_eq!(res, vec![vec![0, 1, 2, 3]]);
    }

    #[test]
    pub fn test_grid_square() {
        let grid = Grid {
            data: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            width: 3,
            height: 3,
        };
        let starts = vec![(1, 1)];
        let res = multi_grid_bfs(&grid, &starts);
        assert_eq!(res, vec![vec![2, 1, 2, 1, 0, 1, 2, 1, 2]]);
    }

    #[test]
    pub fn test_grid_rectangle() {
        let grid = Grid {
            data: vec![
                0, 0, 0, 0, // row 0
                0, 0, 0, 0, // row 1
            ],
            width: 4,
            height: 2,
        };
        let starts = vec![(0, 0)];
        let res = multi_grid_bfs(&grid, &starts);
        assert_eq!(res, vec![vec![0, 1, 2, 3, 1, 2, 3, 4]]);
    }
}
