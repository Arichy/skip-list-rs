use skiplist::SkipList;
use std::collections::{BTreeMap, LinkedList};
use std::time::Instant;

fn main() {
    println!("SkipList vs BTreeMap vs LinkedList - Simple Performance Test");
    println!("============================================================");
    
    let size = 10000;
    println!("Testing with {} elements\n", size);
    
    // Test insertion performance
    println!("1. Insertion Performance:");
    
    let start = Instant::now();
    let mut skiplist = SkipList::new();
    for i in 0..size {
        skiplist.insert(i, i * 2);
    }
    let skiplist_insert_time = start.elapsed();
    println!("   SkipList: {:?}", skiplist_insert_time);
    
    let start = Instant::now();
    let mut btreemap = BTreeMap::new();
    for i in 0..size {
        btreemap.insert(i, i * 2);
    }
    let btreemap_insert_time = start.elapsed();
    println!("   BTreeMap: {:?}", btreemap_insert_time);
    
    let start = Instant::now();
    let mut linkedlist = LinkedList::new();
    for i in 0..size {
        linkedlist.push_back((i, i * 2)); // Sequential insertion is O(1) for LinkedList
    }
    let linkedlist_insert_time = start.elapsed();
    println!("   LinkedList: {:?}", linkedlist_insert_time);
    
    // Test lookup performance
    println!("\n2. Lookup Performance:");
    
    let start = Instant::now();
    for i in 0..size {
        let _ = skiplist.get(&i);
    }
    let skiplist_get_time = start.elapsed();
    println!("   SkipList: {:?}", skiplist_get_time);
    
    let start = Instant::now();
    for i in 0..size {
        let _ = btreemap.get(&i);
    }
    let btreemap_get_time = start.elapsed();
    println!("   BTreeMap: {:?}", btreemap_get_time);
    
    let start = Instant::now();
    for i in 0..size {
        let _ = linkedlist.iter().find(|(k, _)| *k == i);
    }
    let linkedlist_get_time = start.elapsed();
    println!("   LinkedList: {:?}", linkedlist_get_time);

    // Test iteration performance
    println!("\n3. Iteration Performance:");
    
    let start = Instant::now();
    let mut sum = 0;
    for (&k, &v) in &skiplist {
        sum += k + v;
    }
    let skiplist_iter_time = start.elapsed();
    println!("   SkipList: {:?} (sum: {})", skiplist_iter_time, sum);
    
    let start = Instant::now();
    let mut sum = 0;
    for (&k, &v) in &btreemap {
        sum += k + v;
    }
    let btreemap_iter_time = start.elapsed();
    println!("   BTreeMap: {:?} (sum: {})", btreemap_iter_time, sum);
    
    let start = Instant::now();
    let mut sum = 0;
    for (k, v) in &linkedlist {
        sum += k + v;
    }
    let linkedlist_iter_time = start.elapsed();
    println!("   LinkedList: {:?} (sum: {})", linkedlist_iter_time, sum);
    
    // Summary
    println!("\n4. Summary (relative to BTreeMap):");
    println!("   Insert - SkipList: {:.2}x, LinkedList: {:.2}x", 
             skiplist_insert_time.as_nanos() as f64 / btreemap_insert_time.as_nanos() as f64,
             linkedlist_insert_time.as_nanos() as f64 / btreemap_insert_time.as_nanos() as f64);
    println!("   Get    - SkipList: {:.2}x, LinkedList: {:.2}x", 
             skiplist_get_time.as_nanos() as f64 / btreemap_get_time.as_nanos() as f64,
             linkedlist_get_time.as_nanos() as f64 / btreemap_get_time.as_nanos() as f64);
    println!("   Iter   - SkipList: {:.2}x, LinkedList: {:.2}x", 
             skiplist_iter_time.as_nanos() as f64 / btreemap_iter_time.as_nanos() as f64,
             linkedlist_iter_time.as_nanos() as f64 / btreemap_iter_time.as_nanos() as f64);
    
    println!("\nNote: Values > 1.0 mean slower than BTreeMap, < 1.0 mean faster than BTreeMap");
    println!("LinkedList get operations are O(n) and much slower for large datasets");
    println!("For detailed benchmarks with statistical analysis, run: cargo bench");
}