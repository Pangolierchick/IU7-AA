use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/distance.rs"]
mod distance;

static TEST_STRING1: &str = "test";
static TEST_STRING2: &str = "rest";

static TEST_STRING3: &str = "testtesttesttest";
static TEST_STRING4: &str = "restrestrestrest";

/*
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec efficitur.
feugiat ex. Praesent tincidunt semper libero. Mauris finibus nec quam
*/

fn levenstein_rec_bench(c: &mut Criterion) {
    c.bench_function("Lev. recursive", |b| {
        b.iter(|| distance::levenstein_rec(TEST_STRING1, TEST_STRING2))
    });
}

fn levenstein_iter_bench(c: &mut Criterion) {
    c.bench_function("Lev. iteration", |b| {
        b.iter(|| distance::levenstein_iter(TEST_STRING1, TEST_STRING2))
    });
}

fn damerlau_levenstein_rec_bench(c: &mut Criterion) {
    c.bench_function("Damerlau-Lev. recursive", |b| {
        b.iter(|| distance::_damerlau_levenstein_rec(TEST_STRING1, TEST_STRING2))
    });
}

fn damerlau_levenstein_iter_bench(c: &mut Criterion) {
    c.bench_function("Damerlau-Lev. iteration", |b| {
        b.iter(|| distance::damerlau_levenstein_iter(TEST_STRING1, TEST_STRING2))
    });
}

criterion_group!(
    benches,
    levenstein_rec_bench,
    levenstein_iter_bench,
    damerlau_levenstein_rec_bench,
    damerlau_levenstein_iter_bench
);
criterion_main!(benches);
