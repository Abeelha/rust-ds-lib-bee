use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_ds_lib_bee::linear::{LinkedList, Queue, Stack};

fn stack_benchmark(c: &mut Criterion) {
    c.bench_function("stack_push_pop_1000", |b| {
        b.iter(|| {
            let mut stack = Stack::new();
            for i in 0..1000 {
                stack.push(black_box(i));
            }
            for _ in 0..1000 {
                black_box(stack.pop());
            }
        })
    });
}

fn queue_benchmark(c: &mut Criterion) {
    c.bench_function("queue_enqueue_dequeue_1000", |b| {
        b.iter(|| {
            let mut queue = Queue::new();
            for i in 0..1000 {
                queue.enqueue(black_box(i));
            }
            for _ in 0..1000 {
                black_box(queue.dequeue());
            }
        })
    });
}

fn linked_list_benchmark(c: &mut Criterion) {
    c.bench_function("linked_list_push_pop_1000", |b| {
        b.iter(|| {
            let mut list = LinkedList::new();
            for i in 0..1000 {
                list.push_front(black_box(i));
            }
            for _ in 0..1000 {
                black_box(list.pop_front());
            }
        })
    });
}

criterion_group!(
    benches,
    stack_benchmark,
    queue_benchmark,
    linked_list_benchmark
);
criterion_main!(benches);
