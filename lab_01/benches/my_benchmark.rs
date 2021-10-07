use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/distance.rs"]
mod distance;

static TEST_STRING1: &str = "ttttttttt";
static TEST_STRING2: &str = "aaaaaaaaa";

static TEST_STRING3: &str = "ttttttttt";
static TEST_STRING4: &str = "aaaaaaaaa";

/*
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec efficitur.
feugiat ex. Praesent tincidunt semper libero. Mauris finibus nec quam
*/

fn levenstein_rec_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distance");

    group.bench_function("Lev. recursive", move |b| {
        b.iter(|| distance::levenstein_rec(TEST_STRING1, TEST_STRING2))
    });
}

fn levenstein_iter_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distance");

    group.bench_function("Lev. iteration", move |b| {
        b.iter(|| distance::levenstein_iter(TEST_STRING1, TEST_STRING2))
    });
}

fn levenstein_mem_rec_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distance");

    group.bench_function("Lev. mem. rec", move |b| {
        b.iter(|| distance::levenstein_mem_rec(TEST_STRING1, TEST_STRING2))
    });
}

fn damerau_levenstein_rec_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distance");

    group.bench_function("Damerau-Lev. recursive", move |b| {
        b.iter(|| distance::damerau_levenstein_rec(TEST_STRING1, TEST_STRING2))
    });
}

fn damerau_levenstein_iter_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distance");

    group.bench_function("Damerau-Lev. iteration", move |b| {
        b.iter(|| distance::damerau_levenstein_iter(TEST_STRING1, TEST_STRING2))
    });
}

criterion_group!(
    benches,
    levenstein_rec_bench
    // levenstein_iter_bench,
    // levenstein_mem_rec_bench
    // damerau_levenstein_rec_bench,
    // damerau_levenstein_iter_bench
);
criterion_main!(benches);
