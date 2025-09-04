use skiplist::SkipList;

fn main() {
    println!("SkipList Index Method Demo");
    println!("==========================");
    
    let mut skip_list = SkipList::new();
    
    // Insert some elements in random order
    let elements = [42, 17, 8, 23, 4, 15, 31, 6, 19, 12];
    println!("\nInserting elements: {:?}", elements);
    
    for &elem in &elements {
        skip_list.insert(elem, format!("value_{}", elem));
    }
    
    println!("SkipList length: {}", skip_list.len());
    
    // Show elements via iteration (sorted order)
    println!("\nElements via iteration (sorted order):");
    for (i, (&key, value)) in (&skip_list).into_iter().enumerate() {
        println!("  Position {}: {} -> {}", i, key, value);
    }
    
    // Demonstrate index method
    println!("\nDemonstrating index() method:");
    for i in 0..skip_list.len() {
        if let Some((&key, value)) = skip_list.index(i) {
            println!("  index({}): {} -> {}", i, key, value);
        }
    }
    
    // Test out of bounds
    println!("\nOut of bounds access:");
    println!("  index({}): {:?}", skip_list.len(), skip_list.index(skip_list.len()));
    println!("  index(100): {:?}", skip_list.index(100));
    
    // Demonstrate index_mut method
    println!("\nDemonstrating index_mut() method:");
    if let Some((key, value)) = skip_list.index_mut(5) {
        println!("  Before: index(5) = {} -> {}", key, value);
        *value = "MODIFIED".to_string();
        println!("  After:  index(5) = {} -> {}", key, value);
    }
    
    // Remove some elements and show how indexing adapts
    println!("\nRemoving elements [8, 17, 23]:");
    for &elem in &[8, 17, 23] {
        skip_list.remove(&elem);
        println!("  Removed {}, new length: {}", elem, skip_list.len());
    }
    
    println!("\nElements after removal:");
    for i in 0..skip_list.len() {
        if let Some((&key, value)) = skip_list.index(i) {
            println!("  index({}): {} -> {}", i, key, value);
        }
    }
    
    // Performance note
    println!("\nPerformance characteristics:");
    println!("  - index() method uses span information for O(log n) access");
    println!("  - Spans allow efficient skipping through levels");
    println!("  - Much faster than O(n) linear traversal for positional access");
    
    // Large dataset demonstration
    println!("\nLarge dataset test (1000 elements):");
    let mut large_list = SkipList::new();
    for i in 0..1000 {
        large_list.insert(i, i * 10);
    }
    
    // Test random access
    for &index in &[0, 50, 500, 999] {
        if let Some((&key, &value)) = large_list.index(index) {
            println!("  index({}): {} -> {}", index, key, value);
        }
    }
    
    println!("\nDemo completed successfully!");
}