use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::prelude::*;
use skiplist::SkipList;
use std::collections::{BTreeMap, LinkedList};
use std::hint::black_box;

fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        // Benchmark SkipList insertion
        group.bench_with_input(BenchmarkId::new("skiplist", size), size, |b, &size| {
            let mut rng = StdRng::seed_from_u64(42);
            let keys: Vec<i32> = (0..size).map(|_| rng.random_range(0..size * 10)).collect();
            
            b.iter(|| {
                let mut skip_list = SkipList::new();
                for &key in &keys {
                    skip_list.insert(black_box(key), black_box(key * 2));
                }
                skip_list
            });
        });
        
        // Benchmark BTreeMap insertion for comparison
        group.bench_with_input(BenchmarkId::new("btreemap", size), size, |b, &size| {
            let mut rng = StdRng::seed_from_u64(42);
            let keys: Vec<i32> = (0..size).map(|_| rng.random_range(0..size * 10)).collect();
            
            b.iter(|| {
                let mut btree = BTreeMap::new();
                for &key in &keys {
                    btree.insert(black_box(key), black_box(key * 2));
                }
                btree
            });
        });
        
        // Benchmark LinkedList insertion for comparison (note: much slower due to O(n) operations)
        group.bench_with_input(BenchmarkId::new("linkedlist", size), size, |b, &size| {
            let mut rng = StdRng::seed_from_u64(42);
            let keys: Vec<i32> = (0..size).map(|_| rng.random_range(0..size * 10)).collect();
            
            b.iter(|| {
                let mut list: LinkedList<(i32, i32)> = LinkedList::new();
                for &key in &keys {
                    // For fairness, just append to end (O(1)) instead of sorted insertion (O(n))
                    list.push_back((black_box(key), black_box(key * 2)));
                }
                list
            });
        });
    }
    
    group.finish();
}

fn get_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        // Pre-populate data structures
        let mut rng = StdRng::seed_from_u64(42);
        let keys: Vec<i32> = (0..*size).map(|_| rng.random_range(0..size * 10)).collect();
        let lookup_keys: Vec<i32> = (0..*size).map(|_| rng.random_range(0..size * 10)).collect();

        let mut skip_list = SkipList::new();
        let mut btree = BTreeMap::new();
        let mut list: LinkedList<(i32, i32)> = LinkedList::new();
        
        for &key in &keys {
            skip_list.insert(key, key * 2);
            btree.insert(key, key * 2);
            // For LinkedList, just append (we'll use linear search for get)
            list.push_back((key, key * 2));
        }
        
        // Benchmark SkipList get
        group.bench_with_input(BenchmarkId::new("skiplist", size), size, |b, _| {
            b.iter(|| {
                for &key in &lookup_keys {
                    black_box(skip_list.get(&black_box(key)));
                }
            });
        });
        
        // Benchmark BTreeMap get for comparison
        group.bench_with_input(BenchmarkId::new("btreemap", size), size, |b, _| {
            b.iter(|| {
                for &key in &lookup_keys {
                    black_box(btree.get(&black_box(key)));
                }
            });
        });
        
        // Benchmark LinkedList get for comparison (note: O(n) search)
        group.bench_with_input(BenchmarkId::new("linkedlist", size), size, |b, _| {
            b.iter(|| {
                for &key in &lookup_keys {
                    let found = list.iter().find(|(k, _)| *k == black_box(key));
                    black_box(found);
                }
            });
        });
    }
    
    group.finish();
}

fn remove_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("remove");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        // Benchmark SkipList removal
        group.bench_with_input(BenchmarkId::new("skiplist", size), size, |b, &size| {
            let mut rng = StdRng::seed_from_u64(42);
            let keys: Vec<i32> = (0..size).map(|_| rng.random_range(0..size * 10)).collect();
            
            b.iter_batched(
                || {
                    let mut skip_list = SkipList::new();
                    for &key in &keys {
                        skip_list.insert(key, key * 2);
                    }
                    (skip_list, keys.clone())
                },
                |(mut skip_list, keys)| {
                    for &key in &keys {
                        skip_list.remove(&black_box(key));
                    }
                    skip_list
                },
                criterion::BatchSize::SmallInput,
            );
        });
        
        // Benchmark BTreeMap removal for comparison
        group.bench_with_input(BenchmarkId::new("btreemap", size), size, |b, &size| {
            let mut rng = StdRng::seed_from_u64(42);
            let keys: Vec<i32> = (0..size).map(|_| rng.random_range(0..size * 10)).collect();
            
            b.iter_batched(
                || {
                    let mut btree = BTreeMap::new();
                    for &key in &keys {
                        btree.insert(key, key * 2);
                    }
                    (btree, keys.clone())
                },
                |(mut btree, keys)| {
                    for &key in &keys {
                        btree.remove(&black_box(key));
                    }
                    btree
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    
    group.finish();
}

fn iteration_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iteration");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        // Pre-populate data structures
        let mut rng = StdRng::seed_from_u64(42);
        let keys: Vec<i32> = (0..*size).map(|_| rng.random_range(0..size * 10)).collect();
        
        let mut skip_list = SkipList::new();
        let mut btree = BTreeMap::new();
        let mut list: LinkedList<(i32, i32)> = LinkedList::new();
        
        for &key in &keys {
            skip_list.insert(key, key * 2);
            btree.insert(key, key * 2);
            // For LinkedList, just append (we'll use linear search for iteration test)
            list.push_back((key, key * 2));
        }
        
        // Benchmark SkipList borrowed iteration
        group.bench_with_input(BenchmarkId::new("skiplist_borrowed", size), size, |b, _| {
            b.iter(|| {
                let mut sum = 0;
                for (&k, &v) in &skip_list {
                    sum += black_box(k + v);
                }
                sum
            });
        });
        
        // Benchmark SkipList consuming iteration
        group.bench_with_input(BenchmarkId::new("skiplist_consuming", size), size, |b, _| {
            b.iter_batched(
                || {
                    let mut sl = SkipList::new();
                    for &key in &keys {
                        sl.insert(key, key * 2);
                    }
                    sl
                },
                |skip_list| {
                    let mut sum = 0;
                    for (k, v) in skip_list {
                        sum += black_box(k + v);
                    }
                    sum
                },
                criterion::BatchSize::SmallInput,
            );
        });
        
        // Benchmark BTreeMap iteration for comparison
        group.bench_with_input(BenchmarkId::new("btreemap", size), size, |b, _| {
            b.iter(|| {
                let mut sum = 0;
                for (&k, &v) in &btree {
                    sum += black_box(k + v);
                }
                sum
            });
        });
        
        // Benchmark LinkedList iteration for comparison
        group.bench_with_input(BenchmarkId::new("linkedlist", size), size, |b, _| {
            b.iter(|| {
                let mut sum = 0;
                for (k, v) in &list {
                    sum += black_box(k + v);
                }
                sum
            });
        });
    }
    
    group.finish();
}

fn mixed_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_operations");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        // Benchmark SkipList mixed operations
        group.bench_with_input(BenchmarkId::new("skiplist", size), size, |b, &size| {
            let mut rng = StdRng::seed_from_u64(42);
            let operations: Vec<(u8, i32)> = (0..size)
                .map(|_| {
                    let op = rng.random_range(0..3); // 0: insert, 1: get, 2: remove
                    let key = rng.random_range(0..size);
                    (op, key)
                })
                .collect();
            
            b.iter(|| {
                let mut skip_list = SkipList::new();
                for &(op, key) in &operations {
                    match op {
                        0 => { skip_list.insert(black_box(key), black_box(key * 2)); },
                        1 => { black_box(skip_list.get(&black_box(key))); },
                        2 => { black_box(skip_list.remove(&black_box(key))); },
                        _ => unreachable!(),
                    }
                }
                skip_list
            });
        });
        
        // Benchmark BTreeMap mixed operations for comparison
        group.bench_with_input(BenchmarkId::new("btreemap", size), size, |b, &size| {
            let mut rng = StdRng::seed_from_u64(42);
            let operations: Vec<(u8, i32)> = (0..size)
                .map(|_| {
                    let op = rng.random_range(0..3); // 0: insert, 1: get, 2: remove
                    let key = rng.random_range(0..size);
                    (op, key)
                })
                .collect();
            
            b.iter(|| {
                let mut btree = BTreeMap::new();
                for &(op, key) in &operations {
                    match op {
                        0 => { btree.insert(black_box(key), black_box(key * 2)); },
                        1 => { black_box(btree.get(&black_box(key))); },
                        2 => { black_box(btree.remove(&black_box(key))); },
                        _ => unreachable!(),
                    }
                }
                btree
            });
        });
    }
    
    group.finish();
}

fn sequential_vs_random_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_vs_random");
    
    let size = 10000;
    group.throughput(Throughput::Elements(size as u64));
    
    // Sequential insertion
    group.bench_function("skiplist_sequential_insert", |b| {
        b.iter(|| {
            let mut skip_list = SkipList::new();
            for i in 0..size {
                skip_list.insert(black_box(i), black_box(i * 2));
            }
            skip_list
        });
    });
    
    // Random insertion
    group.bench_function("skiplist_random_insert", |b| {
        let mut rng = StdRng::seed_from_u64(42);
        let mut keys: Vec<i32> = (0..size).collect();
        keys.shuffle(&mut rng);
        
        b.iter(|| {
            let mut skip_list = SkipList::new();
            for &key in &keys {
                skip_list.insert(black_box(key), black_box(key * 2));
            }
            skip_list
        });
    });
    
    // Sequential vs Random for BTreeMap comparison
    group.bench_function("btreemap_sequential_insert", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new();
            for i in 0..size {
                btree.insert(black_box(i), black_box(i * 2));
            }
            btree
        });
    });
    
    group.bench_function("btreemap_random_insert", |b| {
        let mut rng = StdRng::seed_from_u64(42);
        let mut keys: Vec<i32> = (0..size).collect();
        keys.shuffle(&mut rng);
        
        b.iter(|| {
            let mut btree = BTreeMap::new();
            for &key in &keys {
                btree.insert(black_box(key), black_box(key * 2));
            }
            btree
        });
    });
    
    // LinkedList comparison (note: much slower due to O(n) insertions)
    group.bench_function("linkedlist_sequential_insert", |b| {
        b.iter(|| {
            let mut list: LinkedList<(i32, i32)> = LinkedList::new();
            for i in 0..size {
                list.push_back((black_box(i), black_box(i * 2)));
            }
            list
        });
    });
    
    group.bench_function("linkedlist_random_insert", |b| {
        let mut rng = StdRng::seed_from_u64(42);
        let mut keys: Vec<i32> = (0..size).collect();
        keys.shuffle(&mut rng);
        
        b.iter(|| {
            let mut list: LinkedList<(i32, i32)> = LinkedList::new();
            for &key in &keys {
                // Just append for LinkedList (not maintaining sorted order for performance)
                list.push_back((black_box(key), black_box(key * 2)));
            }
            list
        });
    });
    
    group.finish();
}

#[cfg(feature = "test-utils")]
fn span_verification_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("span_verification");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("verify_spans", size), size, |b, &size| {
            // Pre-populate skip list
            let mut rng = StdRng::seed_from_u64(42);
            let mut skip_list = SkipList::new();
            for _ in 0..size {
                let key = rng.random_range(0..size * 10);
                skip_list.insert(key, key * 2);
            }
            
            b.iter(|| {
                black_box(skip_list.verify_spans())
            });
        });
    }
    
    group.finish();
}

// Create different benchmark groups based on feature availability
#[cfg(feature = "test-utils")]
criterion_group!(
    benches,
    insert_benchmark,
    get_benchmark,
    remove_benchmark,
    iteration_benchmark,
    mixed_operations_benchmark,
    sequential_vs_random_benchmark,
    span_verification_benchmark
);

#[cfg(not(feature = "test-utils"))]
criterion_group!(
    benches,
    insert_benchmark,
    get_benchmark,
    remove_benchmark,
    iteration_benchmark,
    mixed_operations_benchmark,
    sequential_vs_random_benchmark
);

criterion_main!(benches);