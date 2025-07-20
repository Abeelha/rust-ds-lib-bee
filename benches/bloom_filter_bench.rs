use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_ds_lib_bee::BloomFilter;

fn bench_bloom_filter_insert(c: &mut Criterion) {
    c.bench_function("bloom_filter_insert_1000", |b| {
        let mut filter = BloomFilter::new(1000, 0.01);
        let mut counter = 0;
        b.iter(|| {
            filter.insert(&black_box(counter));
            counter += 1;
        })
    });
}

fn bench_bloom_filter_contains(c: &mut Criterion) {
    let mut filter = BloomFilter::new(1000, 0.01);
    for i in 0..500 {
        filter.insert(&i);
    }

    c.bench_function("bloom_filter_contains", |b| {
        let mut counter = 0;
        b.iter(|| {
            let result = filter.contains(&black_box(counter % 1000));
            counter += 1;
            result
        })
    });
}

fn bench_bloom_filter_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("bloom_filter_sizes");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(format!("insert_{size}"), size, |b, &size| {
            let mut filter = BloomFilter::new(size, 0.01);
            let mut counter = 0;
            b.iter(|| {
                filter.insert(&black_box(counter));
                counter += 1;
            })
        });
    }

    group.finish();
}

fn bench_bloom_filter_false_positive_rates(c: &mut Criterion) {
    let mut group = c.benchmark_group("bloom_filter_fp_rates");

    for &rate in [0.001, 0.01, 0.1].iter() {
        group.bench_with_input(format!("fp_rate_{rate}"), &rate, |b, &rate| {
            let mut filter = BloomFilter::new(1000, rate);
            let mut counter = 0;
            b.iter(|| {
                filter.insert(&black_box(counter));
                counter += 1;
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_bloom_filter_insert,
    bench_bloom_filter_contains,
    bench_bloom_filter_different_sizes,
    bench_bloom_filter_false_positive_rates
);
criterion_main!(benches);
