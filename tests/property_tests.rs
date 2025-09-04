use proptest::prelude::*;
use proptest::strategy::Strategy;
use skiplist::SkipList;
use std::collections::BTreeMap;

proptest! {
    #[test]
    fn test_skiplist_maintains_order(keys in prop::collection::vec(0i32..1000, 0..100)) {
        let mut skip_list = SkipList::new();
        let mut btree = BTreeMap::new();
        
        // Insert all keys
        for key in keys {
            let old_sl = skip_list.insert(key, key * 2);
            let old_bt = btree.insert(key, key * 2);
            prop_assert_eq!(old_sl, old_bt);
            
            // Verify spans after each insertion if test-utils feature is enabled
            #[cfg(feature = "test-utils")]
            prop_assert!(skip_list.verify_spans(), "Span verification failed after inserting key {}", key);
        }
        
        // Verify lengths match
        prop_assert_eq!(skip_list.len(), btree.len());
        
        // Verify all keys are present
        for &key in btree.keys() {
            prop_assert_eq!(skip_list.get(&key), btree.get(&key));
        }
        
        // Verify iteration order matches BTreeMap
        let sl_items: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
        let bt_items: Vec<_> = btree.iter().map(|(&k, &v)| (k, v)).collect();
        prop_assert_eq!(sl_items, bt_items);
        
        // Final span verification
        #[cfg(feature = "test-utils")]
        prop_assert!(skip_list.verify_spans(), "Final span verification failed");
    }

    #[test]
    fn test_skiplist_insertion_and_removal(
        inserts in prop::collection::vec(0i32..100, 0..50),
        removes in prop::collection::vec(0i32..100, 0..25)
    ) {
        let mut skip_list = SkipList::new();
        let mut btree = BTreeMap::new();
        
        // Insert phase
        for key in inserts {
            skip_list.insert(key, key * 3);
            btree.insert(key, key * 3);

            // Verify spans after insertion
            #[cfg(feature = "test-utils")]
            prop_assert!(skip_list.verify_spans(), "Span verification failed after inserting {}", key);
        }
        
        // Remove phase
        for key in removes {
            let sl_result = skip_list.remove(&key);
            let bt_result = btree.remove(&key);
            prop_assert_eq!(sl_result, bt_result);
            
            // Verify spans after removal
            #[cfg(feature = "test-utils")]
            prop_assert!(skip_list.verify_spans(), "Span verification failed after removing {}", key);
        }
        
        // Verify final state
        prop_assert_eq!(skip_list.len(), btree.len());
        
        // Verify remaining elements
        for (&key, &value) in &btree {
            prop_assert_eq!(skip_list.get(&key), Some(&value));
        }
        
        // Verify iteration order
        let sl_items: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
        let bt_items: Vec<_> = btree.iter().map(|(&k, &v)| (k, v)).collect();
        prop_assert_eq!(sl_items, bt_items);
        
        // Final span verification
        #[cfg(feature = "test-utils")]
        prop_assert!(skip_list.verify_spans(), "Final span verification failed after all operations");
    }

    #[test]
    fn test_skiplist_get_operations(
        operations in prop::collection::vec((0i32..50, 0i32..100), 0..100)
    ) {
        let mut skip_list = SkipList::new();
        let mut btree = BTreeMap::new();
        
        for (key, value) in operations {
            skip_list.insert(key, value);
            btree.insert(key, value);
        }
        
        // Test get operations for all possible keys in range
        for test_key in 0..50 {
            prop_assert_eq!(skip_list.get(&test_key), btree.get(&test_key));
        }
    }

    #[test]
    fn test_skiplist_replacement_behavior(
        initial_ops in prop::collection::vec((0i32..20, 0i32..100), 0..30),
        replacement_ops in prop::collection::vec((0i32..20, 0i32..100), 0..15)
    ) {
        let mut skip_list = SkipList::new();
        let mut btree = BTreeMap::new();
        
        // Initial insertions
        for (key, value) in initial_ops {
            skip_list.insert(key, value);
            btree.insert(key, value);
        }
        
        let initial_len = skip_list.len();
        prop_assert_eq!(initial_len, btree.len());
        
        // Verify spans after initial insertions
        #[cfg(feature = "test-utils")]
        prop_assert!(skip_list.verify_spans(), "Span verification failed after initial insertions");
        
        // Replacement operations
        for (key, new_value) in replacement_ops {
            let sl_old = skip_list.insert(key, new_value);
            let bt_old = btree.insert(key, new_value);
            prop_assert_eq!(sl_old, bt_old);
            
            // Verify spans after each replacement
            #[cfg(feature = "test-utils")]
            prop_assert!(skip_list.verify_spans(), "Span verification failed after replacing key {} with value {}", key, new_value);
        }
        
        // Length should not exceed initial length plus new unique keys
        prop_assert_eq!(skip_list.len(), btree.len());
        
        // Verify all final values
        for (&key, &value) in &btree {
            prop_assert_eq!(skip_list.get(&key), Some(&value));
        }
        
        // Final span verification
        #[cfg(feature = "test-utils")]
        prop_assert!(skip_list.verify_spans(), "Final span verification failed after replacements");
    }

    #[test]
    fn test_skiplist_iterator_completeness(keys in prop::collection::vec(0i32..100, 0..50)) {
        let mut skip_list = SkipList::new();
        
        // Insert unique keys only
        let mut unique_keys = keys;
        unique_keys.sort();
        unique_keys.dedup();
        
        for &key in &unique_keys {
            skip_list.insert(key, key);
        }
        
        // Test borrowed iterator
        let borrowed_items: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
        let expected_items: Vec<_> = unique_keys.iter().map(|&k| (k, k)).collect();
        prop_assert_eq!(borrowed_items, expected_items.clone());
        
        // Test consuming iterator
        let consuming_items: Vec<_> = skip_list.into_iter().collect();
        prop_assert_eq!(consuming_items, expected_items);
    }

    #[test]
    fn test_skiplist_stress_operations(
        ops in prop::collection::vec(
            (0i32..50, 0i32..100, 0i32..3).prop_map(|(k, v, op_type)| {
                match op_type {
                    0 => ("insert", k, v),
                    1 => ("remove", k, 0),
                    _ => ("get", k, 0),
                }
            }),
            0..200
        )
    ) {
        let mut skip_list = SkipList::new();
        let mut btree = BTreeMap::new();
        
        for (op, key, _value) in ops {
            match op {
                "insert" => {
                    let sl_result = skip_list.insert(key, key * 2); // Use key * 2 as value
                    let bt_result = btree.insert(key, key * 2);
                    prop_assert_eq!(sl_result, bt_result);
                    
                    // Verify spans after insertion
                    #[cfg(feature = "test-utils")]
                    prop_assert!(skip_list.verify_spans(), "Span verification failed after inserting {}", key);
                },
                "remove" => {
                    let sl_result = skip_list.remove(&key);
                    let bt_result = btree.remove(&key);
                    prop_assert_eq!(sl_result, bt_result);
                    
                    // Verify spans after removal
                    #[cfg(feature = "test-utils")]
                    prop_assert!(skip_list.verify_spans(), "Span verification failed after removing {}", key);
                },
                "get" => {
                    let sl_result = skip_list.get(&key);
                    let bt_result = btree.get(&key);
                    prop_assert_eq!(sl_result, bt_result);
                    // No span verification needed for get operations
                },
                _ => unreachable!(),
            }
            
            // Invariant: lengths should always match
            prop_assert_eq!(skip_list.len(), btree.len());
        }
        
        // Final verification: all elements match
        let sl_items: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
        let bt_items: Vec<_> = btree.iter().map(|(&k, &v)| (k, v)).collect();
        prop_assert_eq!(sl_items, bt_items);
        
        // Final comprehensive span verification
        #[cfg(feature = "test-utils")]
        prop_assert!(skip_list.verify_spans(), "Final span verification failed after stress test");
    }

    #[test]
    #[cfg(feature = "test-utils")]
    fn test_span_consistency_property(
        operations in prop::collection::vec((0i32..50, 0i32..100), 0..100)
    ) {
        let mut skip_list = SkipList::new();
        
        // After each operation, verify spans are consistent
        for (key, value) in operations {
            skip_list.insert(key, value);
            prop_assert!(skip_list.verify_spans(), "Span verification failed after inserting ({}, {})", key, value);
        }
        
        // Test removals while verifying spans
        let keys_to_remove: Vec<_> = (0..50).step_by(3).collect();
        for key in keys_to_remove {
            skip_list.remove(&key);
            prop_assert!(skip_list.verify_spans(), "Span verification failed after removing {}", key);
        }
    }

    #[test]
    fn test_empty_and_single_element_operations(
        key in 0i32..100,
        value in 0i32..1000
    ) {
        // Test empty skip list
        let empty_list: SkipList<i32, i32> = SkipList::new();
        prop_assert_eq!(empty_list.len(), 0);
        prop_assert_eq!(empty_list.get(&key), None);
        
        let items: Vec<_> = (&empty_list).into_iter().collect();
        prop_assert_eq!(items, vec![]);
        
        let items: Vec<_> = empty_list.into_iter().collect();
        prop_assert_eq!(items, vec![]);
        
        // Test single element
        let mut single_list = SkipList::new();
        prop_assert_eq!(single_list.insert(key, value), None);
        prop_assert_eq!(single_list.len(), 1);
        prop_assert_eq!(single_list.get(&key), Some(&value));
        
        let items: Vec<_> = (&single_list).into_iter().map(|(&k, &v)| (k, v)).collect();
        prop_assert_eq!(items, vec![(key, value)]);
        
        prop_assert_eq!(single_list.remove(&key), Some(value));
        prop_assert_eq!(single_list.len(), 0);
    }
}

// Additional non-proptest tests for specific span verification
#[test]
fn test_span_consistency_manual() {
    let mut skip_list = SkipList::new();
    
    // Insert some elements
    for i in [1, 3, 5, 7, 9] {
        skip_list.insert(i, i * 10);
        
        // Verify spans after each insertion if test-utils feature is available
        #[cfg(feature = "test-utils")]
        assert!(skip_list.verify_spans(), "Span verification failed after inserting {}", i);
    }
    
    // The span verification would require access to internal structure
    // For now, we verify that the skip list maintains correct ordering
    let keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    assert_eq!(keys, vec![1, 3, 5, 7, 9]);
    
    // Remove middle element and verify ordering is maintained
    skip_list.remove(&5);
    
    // Verify spans after removal
    #[cfg(feature = "test-utils")]
    assert!(skip_list.verify_spans(), "Span verification failed after removing 5");
    
    let keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    assert_eq!(keys, vec![1, 3, 7, 9]);
}

#[test]
fn test_large_dataset_property() {
    let mut skip_list = SkipList::new();
    let mut expected_keys = Vec::new();
    
    // Insert 1000 elements
    for i in 0..1000 {
        let key = (i * 17) % 500; // Some duplicates
        skip_list.insert(key, key);
        if !expected_keys.contains(&key) {
            expected_keys.push(key);
        }
        
        // Verify spans periodically to avoid performance issues
        #[cfg(feature = "test-utils")]
        if i % 100 == 0 {
            assert!(skip_list.verify_spans(), "Span verification failed after inserting {} elements", i + 1);
        }
    }
    
    expected_keys.sort();
    
    let actual_keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    assert_eq!(actual_keys, expected_keys);
    assert_eq!(skip_list.len(), expected_keys.len());
    
    // Final comprehensive span verification
    #[cfg(feature = "test-utils")]
    assert!(skip_list.verify_spans(), "Final span verification failed for large dataset");
}