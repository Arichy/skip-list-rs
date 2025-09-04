#![cfg(feature = "test-utils")]
use skiplist::SkipList;

#[test]
fn test_span_verification_basic() {
    let mut skip_list = SkipList::new();
    
    // Test empty list
    assert!(skip_list.verify_spans());
    
    // Test single element
    skip_list.insert(5, 50);
    assert!(skip_list.verify_spans());
    
    // Test multiple elements
    for i in [1, 3, 7, 9, 11] {
        skip_list.insert(i, i * 10);
        assert!(skip_list.verify_spans(), "Span verification failed after inserting {}", i);
    }
}

#[test]
fn test_span_verification_with_removals() {
    let mut skip_list = SkipList::new();
    
    // Insert elements
    for i in 1..=10 {
        skip_list.insert(i, i * 10);
        assert!(skip_list.verify_spans(), "Span verification failed after inserting {}", i);
    }
    
    // Remove elements one by one
    for i in [2, 4, 6, 8, 10] {
        skip_list.remove(&i);
        assert!(skip_list.verify_spans(), "Span verification failed after removing {}", i);
    }
    
    // Remove remaining elements
    for i in [1, 3, 5, 7, 9] {
        skip_list.remove(&i);
        assert!(skip_list.verify_spans(), "Span verification failed after removing {}", i);
    }
}

#[test]
fn test_span_verification_with_replacements() {
    let mut skip_list = SkipList::new();

    // Insert initial elements
    for i in [5, 10, 15, 20, 25] {
        skip_list.insert(i, i);
        assert!(skip_list.verify_spans());
    }
    
    // Replace values (should not affect spans)
    for i in [5, 10, 15, 20, 25] {
        skip_list.insert(i, i * 100);
        assert!(skip_list.verify_spans(), "Span verification failed after replacing value for key {}", i);
    }
}

#[test]
fn test_span_verification_stress() {
    let mut skip_list = SkipList::new();
    
    // Insert many elements in random order
    let elements = [23, 7, 45, 12, 89, 3, 56, 34, 78, 1, 99, 67, 21, 43, 88, 15, 92, 4, 76, 29];
    
    for &elem in &elements {
        skip_list.insert(elem, elem);
        assert!(skip_list.verify_spans(), "Span verification failed after inserting {}", elem);
    }
    
    // Remove half the elements
    for &elem in elements.iter().step_by(2) {
        skip_list.remove(&elem);
        assert!(skip_list.verify_spans(), "Span verification failed after removing {}", elem);
    }
    
    // Insert some new elements
    for &elem in &[100, 101, 102, 103, 104] {
        skip_list.insert(elem, elem);
        assert!(skip_list.verify_spans(), "Span verification failed after inserting {}", elem);
    }
}

#[test]
fn test_span_verification_edge_cases() {
    let mut skip_list = SkipList::new();
    
    // Insert and immediately remove
    skip_list.insert(42, 42);
    assert!(skip_list.verify_spans());
    skip_list.remove(&42);
    assert!(skip_list.verify_spans());
    
    // Insert same key multiple times
    skip_list.insert(10, 100);
    assert!(skip_list.verify_spans());
    skip_list.insert(10, 200);
    assert!(skip_list.verify_spans());
    skip_list.insert(10, 300);
    assert!(skip_list.verify_spans());
    
    // Should still have only one element
    assert_eq!(skip_list.len(), 1);
    assert_eq!(skip_list.get(&10), Some(&300));
}

#[test]
fn test_comprehensive_operations_with_span_check() {
    let mut skip_list = SkipList::new();
    
    // Test sequence of complex operations
    let operations = [
        ("insert", 50, 500),
        ("insert", 25, 250),
        ("insert", 75, 750),
        ("insert", 12, 120),
        ("insert", 37, 370),
        ("insert", 62, 620),
        ("insert", 87, 870),
        ("remove", 25, 0),
        ("insert", 30, 300),
        ("remove", 75, 0),
        ("insert", 80, 800),
        ("remove", 12, 0),
        ("insert", 15, 150),
    ];
    
    for (op, key, value) in operations {
        match op {
            "insert" => {
                skip_list.insert(key, value);
            },
            "remove" => {
                skip_list.remove(&key);
            },
            _ => panic!("Unknown operation"),
        }
        assert!(skip_list.verify_spans(), "Span verification failed after {} {}", op, key);
    }
    
    // Final verification - elements should be in sorted order
    let keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    let mut sorted_keys = keys.clone();
    sorted_keys.sort();
    assert_eq!(keys, sorted_keys, "Final iteration is not in sorted order");
}

#[test]
fn test_span_verification_large_dataset() {
    let mut skip_list = SkipList::new();
    
    // Insert a larger dataset to test efficiency of new verify_spans implementation
    for i in 0..1000 {
        let key = (i * 7) % 500; // Creates some duplicates and non-sequential order
        skip_list.insert(key, key);
        
        // Verify spans occasionally (not every insert for performance)
        if i % 100 == 0 {
            assert!(skip_list.verify_spans(), "Span verification failed after inserting {} elements", i + 1);
        }
    }
    
    // Final verification
    assert!(skip_list.verify_spans(), "Final span verification failed");
    
    // Remove some elements and verify again
    for i in 0..100 {
        skip_list.remove(&(i * 3));
        if i % 25 == 0 {
            assert!(skip_list.verify_spans(), "Span verification failed after removing {} elements", i + 1);
        }
    }
}