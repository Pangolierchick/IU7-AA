use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
#[path = "../src/matrix.rs"]
mod matrix;

fn generate_rnd_matrix(size : usize) -> matrix::Matrix<i32> {
    let mut m : matrix::Matrix<i32> = matrix::Matrix::new_zero(size, size);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..size {
            m[[i, j]] = rng.gen::<i32>() % 1010;
        }
    }
    m
}

fn canon_bench_101(c: &mut Criterion) {
    let mut group = c.benchmark_group("Canonical");
    let SIZE = 101;

    group.sample_size(25);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Canon 101", move |b| {
        b.iter(|| matrix::Matrix::default_mult(&m1, &m2))
    });
}

fn canon_bench_201(c: &mut Criterion) {
    let mut group = c.benchmark_group("Canonical");
    let SIZE = 201;

    group.sample_size(25);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Canon 201", move |b| {
        b.iter(|| matrix::Matrix::default_mult(&m1, &m2))
    });
}

fn canon_bench_401(c: &mut Criterion) {
    let mut group = c.benchmark_group("Canonical");
    let SIZE = 401;

    group.sample_size(15);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Canon 401", move |b| {
        b.iter(|| matrix::Matrix::default_mult(&m1, &m2))
    });
}

fn canon_bench_801(c: &mut Criterion) {
    let mut group = c.benchmark_group("Canonical");
    let SIZE = 801;

    group.sample_size(15);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Canon 801", move |b| {
        b.iter(|| matrix::Matrix::default_mult(&m1, &m2))
    });
}

// =========================================================================

fn vinograd_bench_101(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vinograd");
    let SIZE = 101;

    group.sample_size(25);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Vinograd 101", move |b| {
        b.iter(|| matrix::Matrix::vinograd_mult(&m1, &m2))
    });
}

fn vinograd_bench_201(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vinograd");
    let SIZE = 201;

    group.sample_size(25);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Vinograd 201", move |b| {
        b.iter(|| matrix::Matrix::vinograd_mult(&m1, &m2))
    });
}

fn vinograd_bench_401(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vinograd");
    let SIZE = 401;

    group.sample_size(15);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Vinograd 401", move |b| {
        b.iter(|| matrix::Matrix::vinograd_mult(&m1, &m2))
    });
}

fn vinograd_bench_801(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vinograd");
    let SIZE = 801;

    group.sample_size(15);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Vinograd 801", move |b| {
        b.iter(|| matrix::Matrix::vinograd_mult(&m1, &m2))
    });
}

// =========================================================================

fn vinograd_opt_bench_101(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vinograd");
    let SIZE = 101;

    group.sample_size(25);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Vinograd opt 101", move |b| {
        b.iter(|| matrix::Matrix::vinograd_opt_mult(&m1, &m2))
    });
}

fn vinograd_opt_bench_201(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vinograd");
    let SIZE = 201;

    group.sample_size(25);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Vinograd opt 201", move |b| {
        b.iter(|| matrix::Matrix::vinograd_opt_mult(&m1, &m2))
    });
}

fn vinograd_opt_bench_401(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vinograd");
    let SIZE = 401;

    group.sample_size(15);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Vinograd opt 401", move |b| {
        b.iter(|| matrix::Matrix::vinograd_opt_mult(&m1, &m2))
    });
}

fn vinograd_opt_bench_801(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vinograd");
    let SIZE = 801;

    group.sample_size(15);

    let m1 = generate_rnd_matrix(SIZE);
    let m2 = generate_rnd_matrix(SIZE);

    group.bench_function("Vinograd opt 801", move |b| {
        b.iter(|| matrix::Matrix::vinograd_opt_mult(&m1, &m2))
    });
}

criterion_group!(
    benches,
    canon_bench_101,
    canon_bench_201,
    canon_bench_401,
    canon_bench_801,
    vinograd_bench_101,
    vinograd_bench_201,
    vinograd_bench_401,
    vinograd_bench_801,
    vinograd_opt_bench_101,
    vinograd_opt_bench_201,
    vinograd_opt_bench_401,
    vinograd_opt_bench_801
);

criterion_main!(benches);
