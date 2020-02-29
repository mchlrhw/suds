use criterion::{black_box, criterion_group, criterion_main, Criterion};

use suds::Grid;

fn solve_benchmarks(c: &mut Criterion) {
    c.bench_function("backtracking_solve", |b| {
        b.iter(|| black_box(Grid::empty()).backtracking_solve())
    });
}

criterion_group!(benches, solve_benchmarks);
criterion_main!(benches);
