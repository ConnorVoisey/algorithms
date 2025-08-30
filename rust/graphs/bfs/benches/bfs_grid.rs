use bfs::*;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_multi_bfs(c: &mut Criterion) {
    let n = 100_000;
    let mut adj: Vec<Vec<i32>> = vec![Vec::new(); n];
    for i in 0..n-1 {
        adj[i].push((i+1) as i32);
        adj[i+1].push(i as i32);
    }
    let starts: Vec<_> = (0..100).collect();

    c.bench_function("multi_bfs 100 starts, 100k nodes", |b| {
        b.iter(|| {
            let _ = black_box(multi_bfs(&adj, &starts));
        });
    });
}

fn bench_multi_grid_bfs(c: &mut Criterion) {
    let rows = 500;
    let cols = 500;
    let grid = Grid { data: vec![0; rows*cols], width: cols, height: rows };
    let starts: Vec<_> = (0..50).map(|i| (i, i)).collect();

    c.bench_function("multi_grid_bfs 50 starts, 500x500 grid", |b| {
        b.iter(|| {
            let _ = black_box(multi_grid_bfs(&grid, &starts));
        });
    });
}

criterion_group!(benches, bench_multi_bfs, bench_multi_grid_bfs);
criterion_main!(benches);

