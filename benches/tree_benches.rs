use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_ds_lib_bee::{BinarySearchTree, HashMap};

fn bst_insert_benchmark(c: &mut Criterion) {
    c.bench_function("bst_insert_1000", |b| {
        b.iter(|| {
            let mut tree = BinarySearchTree::new();
            for i in 0..1000 {
                tree.insert(black_box(i));
            }
            black_box(tree);
        })
    });
}

fn bst_search_benchmark(c: &mut Criterion) {
    let mut tree = BinarySearchTree::new();
    for i in 0..1000 {
        tree.insert(i);
    }

    c.bench_function("bst_search_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(tree.contains(&black_box(i)));
            }
        })
    });
}

fn bst_iter_benchmark(c: &mut Criterion) {
    let mut tree = BinarySearchTree::new();
    for i in 0..1000 {
        tree.insert(i);
    }

    c.bench_function("bst_iter_1000", |b| {
        b.iter(|| {
            for value in tree.iter() {
                black_box(value);
            }
        })
    });
}

fn hashmap_insert_benchmark(c: &mut Criterion) {
    c.bench_function("hashmap_insert_1000", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(black_box(i), black_box(i * 10));
            }
            black_box(map);
        })
    });
}

fn hashmap_get_benchmark(c: &mut Criterion) {
    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert(i, i * 10);
    }

    c.bench_function("hashmap_get_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(map.get(&black_box(i)));
            }
        })
    });
}

fn hashmap_collision_benchmark(c: &mut Criterion) {
    c.bench_function("hashmap_collision_handling", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(8);
            for i in 0..100 {
                map.insert(black_box(i), black_box(i * 10));
            }
            for i in 0..100 {
                black_box(map.get(&black_box(i)));
            }
        })
    });
}

criterion_group!(
    benches,
    bst_insert_benchmark,
    bst_search_benchmark,
    bst_iter_benchmark,
    hashmap_insert_benchmark,
    hashmap_get_benchmark,
    hashmap_collision_benchmark
);
criterion_main!(benches);
