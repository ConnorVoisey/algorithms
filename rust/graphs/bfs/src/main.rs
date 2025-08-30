use bfs::{Grid, multi_bfs, multi_grid_bfs};
use rayon::ThreadPoolBuilder;
use std::{
    io::{self, Write},
    time::Instant,
};

fn main() {
    println!("Enter max threads (or leave blank to use max hardware concurrency): ");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();

    let max_threads = if line.is_empty() {
        println!("Using hardware concurrency to determine thread count");
        num_cpus::get()
    } else {
        match line.parse::<usize>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid number! Falling back to hardware concurrency.");
                num_cpus::get()
            }
        }
    };

    println!("Using max threads = {max_threads}");
    println!("Starting now...");

    let pool = ThreadPoolBuilder::new()
        .num_threads(max_threads)
        .build()
        .unwrap();

    pool.install(|| {
        grid_stress_test();
        adj_stress_test();
    });
}

fn grid_stress_test() {
    let rows = 1_000;
    let cols = 1_000;

    let grid = Grid {
        data: vec![0; rows * cols],
        width: cols,
        height: rows,
    };

    let mut grid_starts = Vec::with_capacity(1000);
    for i in 0..1000 {
        grid_starts.push((i, i));
    }

    let t1 = Instant::now();
    let _grid_results = multi_grid_bfs(&grid, &grid_starts);
    let t2 = t1.elapsed();

    println!(
        "multi_grid_bfs ({} starts, {} cells) took {:.3?}",
        1000,
        rows * cols,
        t2
    );
}

fn adj_stress_test() {
    let n = 1_000_000;
    let mut adj: Vec<Vec<i32>> = vec![Vec::new(); n];

    // Sparse chain + extra edges
    for i in 0..n - 1 {
        adj[i].push((i + 1) as i32);
        adj[i + 1].push(i as i32);
        if i.is_multiple_of(1000) && i + 100 < n {
            adj[i].push((i + 100) as i32);
            adj[i + 100].push(i as i32);
        }
    }

    let mut adj_starts = Vec::with_capacity(1000);
    for i in 0..1000 {
        adj_starts.push(i * 1000);
    }

    let t1 = Instant::now();
    let _adj_results = multi_bfs(&adj, &adj_starts);
    let t2 = t1.elapsed();

    println!("multi_bfs ({} starts, {} nodes) took {:.3?}", 1000, n, t2);
}
