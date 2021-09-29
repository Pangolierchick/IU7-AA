use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
#[path="../src/sorts.rs"]
mod sorts;

fn bubble_10_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let arr_10 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    group.bench_function("Bubble sort 10 best", move |b| {
        b.iter(|| { let mut c_arr_10 = arr_10.clone(); sorts::bubble_sort(& mut c_arr_10) })
    });
}

fn bubble_100_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut arr_100 = vec![0;100];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Bubble sort 100 best", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::bubble_sort(& mut c_arr_100) })
    });
}

fn bubble_500_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut arr_100 = vec![0;500];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Bubble sort 500 best", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::bubble_sort(& mut c_arr_100) })
    });
}

fn bubble_1000_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut arr_1000 = vec![0;1000];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Bubble sort 1000 best", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::bubble_sort(& mut c_arr_1000) })
    });
}

fn bubble_10_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let arr_10 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    group.bench_function("Bubble sort 10 worst", move |b| {
        b.iter(|| { let mut c_arr_10 = arr_10.clone(); sorts::bubble_sort(& mut c_arr_10) })
    });
}

fn bubble_100_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut arr_100 = vec![0;100];
    let len = arr_100.len();

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Bubble sort 100 worst", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::bubble_sort(& mut c_arr_100) })
    });
}


fn bubble_500_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut arr_100 = vec![0;500];
    let len = arr_100.len();

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Bubble sort 500 worst", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::bubble_sort(& mut c_arr_100) })
    });
}

fn bubble_1000_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut arr_1000 = vec![0;1000];
    let len = arr_1000.len();

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Bubble sort 1000 worst", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::bubble_sort(& mut c_arr_1000) })
    });
}

fn insertion_10_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let arr_10 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    group.bench_function("Insertion sort 10 best", move |b| {
        b.iter(|| { let mut c_arr_10 = arr_10.clone(); sorts::insertion_sort(& mut c_arr_10) })
    });
}

fn insertion_100_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut arr_100 = vec![0;100];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Insertion sort 100 best", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::insertion_sort(& mut c_arr_100) })
    });
}

fn insertion_500_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut arr_100 = vec![0;500];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Insertion sort 500 best", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::insertion_sort(& mut c_arr_100) })
    });
}

fn insertion_1000_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut arr_1000 = vec![0;1000];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Insertion sort 1000 best", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::insertion_sort(& mut c_arr_1000) })
    });
}

fn insertion_10_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let arr_10 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    group.bench_function("Insertion sort 10 worst", move |b| {
        b.iter(|| { let mut c_arr_10 = arr_10.clone(); sorts::insertion_sort(& mut c_arr_10) })
    });
}

fn insertion_100_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut arr_100 = vec![0;100];
    let len = arr_100.len();

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Insertion sort 100 worst", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::insertion_sort(& mut c_arr_100) })
    });
}

fn insertion_500_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut arr_100 = vec![0;500];
    let len = arr_100.len();

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Insertion sort 500 worst", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::insertion_sort(& mut c_arr_100) })
    });
}

fn insertion_1000_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut arr_1000 = vec![0;1000];
    let len = arr_1000.len();

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Insertion sort 1000 worst", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::insertion_sort(& mut c_arr_1000) })
    });
}

fn selection_10_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let arr_10 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    group.bench_function("Selection sort 10 best", move |b| {
        b.iter(|| { let mut c_arr_10 = arr_10.clone(); sorts::selection_sort(& mut c_arr_10) })
    });
}

fn selection_100_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut arr_100 = vec![0;100];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Selection sort 100 best", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::selection_sort(& mut c_arr_100) })
    });
}

fn selection_500_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut arr_100 = vec![0;500];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Selection sort 500 best", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::selection_sort(& mut c_arr_100) })
    });
}

fn selection_1000_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut arr_1000 = vec![0;1000];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = i;
    }

    group.bench_function("Selection sort 1000 best", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::selection_sort(& mut c_arr_1000) })
    });
}

fn selection_10_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let arr_10 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    group.bench_function("Selection sort 10 worst", move |b| {
        b.iter(|| { let mut c_arr_10 = arr_10.clone(); sorts::selection_sort(& mut c_arr_10) })
    });
}

fn selection_100_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut arr_100 = vec![0;100];
    let len = arr_100.len();

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Selection sort 100 worst", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::selection_sort(& mut c_arr_100) })
    });
}

fn selection_500_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut arr_100 = vec![0;500];
    let len = arr_100.len();

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Selection sort 500 worst", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::selection_sort(& mut c_arr_100) })
    });
}

fn selection_1000_worst(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut arr_1000 = vec![0;1000];
    let len = arr_1000.len();

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = len - i;
    }

    group.bench_function("Selection sort 1000 worst", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::selection_sort(& mut c_arr_1000) })
    });
}

fn bubble_100_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut rng = rand::thread_rng();

    let mut arr_100 = vec![0;100];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Bubble sort 100 rand", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::bubble_sort(& mut c_arr_100) })
    });
}

fn bubble_1000_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut rng = rand::thread_rng();

    let mut arr_1000 = vec![0;1000];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Bubble sort 1000 rand", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::bubble_sort(& mut c_arr_1000) })
    });
}

fn bubble_2500_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut rng = rand::thread_rng();

    let mut arr_1000 = vec![0;2500];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Bubble sort 2500 rand", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::bubble_sort(& mut c_arr_1000) })
    });
}

fn bubble_5000_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble sort");

    let mut rng = rand::thread_rng();

    let mut arr_5000 = vec![0;5000];

    for (i, item) in arr_5000.iter_mut().enumerate() {
        *item = rng.gen();
    }


    group.bench_function("Bubble sort 5000 rand", move |b| {
        b.iter(|| { let mut c_arr_5000 = arr_5000.clone(); sorts::bubble_sort(& mut c_arr_5000) })
    });
}

fn insertion_100_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut rng = rand::thread_rng();

    let mut arr_100 = vec![0;100];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Insertion sort 100 rand", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::insertion_sort(& mut c_arr_100) })
    });
}

fn insertion_1000_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut rng = rand::thread_rng();

    let mut arr_1000 = vec![0;1000];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Insertion sort 1000 rand", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::insertion_sort(& mut c_arr_1000) })
    });
}

fn insertion_2500_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut rng = rand::thread_rng();

    let mut arr_1000 = vec![0;2500];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Insertion sort 2500 rand", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::insertion_sort(& mut c_arr_1000) })
    });
}

fn insertion_5000_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    let mut rng = rand::thread_rng();

    let mut arr_5000 = vec![0;5000];

    for (i, item) in arr_5000.iter_mut().enumerate() {
        *item = rng.gen();
    }


    group.bench_function("Insertion sort 5000 rand", move |b| {
        b.iter(|| { let mut c_arr_5000 = arr_5000.clone(); sorts::insertion_sort(& mut c_arr_5000) })
    });
}

fn selection_100_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut rng = rand::thread_rng();

    let mut arr_100 = vec![0;100];

    for (i, item) in arr_100.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Selection sort 100 rand", move |b| {
        b.iter(|| { let mut c_arr_100 = arr_100.clone(); sorts::selection_sort(& mut c_arr_100) })
    });
}

fn selection_1000_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut rng = rand::thread_rng();

    let mut arr_1000 = vec![0;1000];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Selection sort 1000 rand", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::selection_sort(& mut c_arr_1000) })
    });
}

fn selection_2500_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut rng = rand::thread_rng();

    let mut arr_1000 = vec![0;2500];

    for (i, item) in arr_1000.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Selection sort 2500 rand", move |b| {
        b.iter(|| { let mut c_arr_1000 = arr_1000.clone(); sorts::selection_sort(& mut c_arr_1000) })
    });
}


fn selection_5000_rand(c: &mut Criterion) {
    let mut group = c.benchmark_group("Selection sort");

    let mut rng = rand::thread_rng();

    let mut arr_5000 = vec![0;5000];

    for (i, item) in arr_5000.iter_mut().enumerate() {
        *item = rng.gen();
    }

    group.bench_function("Selection sort 5000 rand", move |b| {
        b.iter(|| { let mut c_arr_5000 = arr_5000.clone(); sorts::selection_sort(& mut c_arr_5000) })
    });
}

criterion_group!(
    benches,
    // bubble_10_best,
    // bubble_100_best,
    // bubble_500_best,
    // bubble_1000_best,
    // bubble_10_worst,
    // bubble_100_worst,
    // bubble_500_worst,
    // bubble_1000_worst,
    // insertion_10_best,
    // insertion_100_best,
    // insertion_500_best,
    // insertion_1000_best,
    // insertion_10_worst,
    // insertion_100_worst,
    // insertion_500_worst,
    // insertion_1000_worst,
    // selection_10_best,
    // selection_100_best,
    // selection_500_best,
    // selection_1000_best,
    // selection_10_worst,
    // selection_100_worst,
    // selection_500_worst,
    // selection_1000_worst,
    // bubble_100_rand,
    // bubble_1000_rand,
    // bubble_2500_rand,
    bubble_5000_rand,
    // insertion_100_rand,
    // insertion_1000_rand,
    // insertion_2500_rand,
    insertion_5000_rand,
    // selection_100_rand,
    // selection_1000_rand,
    // selection_2500_rand,
    selection_5000_rand
);
criterion_main!(benches);
