use skiplist::SkipList;
use std::collections::BTreeSet;

#[test]
fn test_basic_operations() {
    let mut skip_list = SkipList::new();

    // Test insertion
    assert_eq!(skip_list.insert("b", 2), None);
    assert_eq!(skip_list.insert("a", 1), None);
    assert_eq!(skip_list.insert("c", 3), None);
    assert_eq!(skip_list.len(), 3);

    // Test replacement
    assert_eq!(skip_list.insert("b", 20), Some(2));
    assert_eq!(skip_list.len(), 3);

    // Test retrieval
    assert_eq!(skip_list.get(&"a"), Some(&1));
    assert_eq!(skip_list.get(&"b"), Some(&20));
    assert_eq!(skip_list.get(&"c"), Some(&3));
    assert_eq!(skip_list.get(&"d"), None);

    // Test mutable retrieval
    assert_eq!(skip_list.get_mut(&"b"), Some(&mut 20));
    if let Some(val) = skip_list.get_mut(&"b") {
        *val = 200;
    }
    assert_eq!(skip_list.get(&"b"), Some(&200));

    // Test removal
    assert_eq!(skip_list.remove(&"b"), Some(200));
    assert_eq!(skip_list.len(), 2);
    assert_eq!(skip_list.get(&"b"), None);

    // Test removal of non-existent key
    assert_eq!(skip_list.remove(&"d"), None);
    assert_eq!(skip_list.len(), 2);
}

#[test]
fn test_ordering() {
    let mut skip_list = SkipList::new();
    let mut expected = BTreeSet::new();
    
    // Insert random order
    let keys = [5, 2, 8, 1, 7, 3, 9, 4, 6];
    for &key in &keys {
        skip_list.insert(key, key * 10);
        expected.insert((key, key * 10));
    }

    // Collect items using borrowed iterator
    let items: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
    let expected_items: Vec<_> = expected.iter().map(|&(k, v)| (k, v)).collect();
    
    assert_eq!(items, expected_items);

    // Test that keys are in sorted order
    let keys_only: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    let mut sorted_keys = keys_only.clone();
    sorted_keys.sort();
    assert_eq!(keys_only, sorted_keys);
}

#[test]
fn test_edge_cases() {
    let mut skip_list = SkipList::new();

    // Test empty skip list
    assert_eq!(skip_list.len(), 0);
    assert_eq!(skip_list.get(&1), None);
    assert_eq!(skip_list.remove(&1), None);

    // Test single element
    skip_list.insert(42, "answer");
    assert_eq!(skip_list.len(), 1);
    assert_eq!(skip_list.get(&42), Some(&"answer"));
    
    // Test removal of single element
    assert_eq!(skip_list.remove(&42), Some("answer"));
    assert_eq!(skip_list.len(), 0);
    assert_eq!(skip_list.get(&42), None);

    // Test duplicate insertions
    skip_list.insert(1, "one");
    skip_list.insert(1, "ONE");
    assert_eq!(skip_list.len(), 1);
    assert_eq!(skip_list.get(&1), Some(&"ONE"));
}

#[test]
fn test_large_dataset() {
    let mut skip_list = SkipList::new();
    let mut expected = BTreeSet::new();
    
    // Insert 1000 random elements
    for i in 0..1000 {
        let key = (i * 7) % 1000; // Creates some duplicates
        let old_val = skip_list.insert(key, key * 2);
        
        if expected.contains(&key) {
            assert!(old_val.is_some());
        } else {
            assert!(old_val.is_none());
        }
        expected.insert(key);
    }

    assert_eq!(skip_list.len(), expected.len());

    // Verify all elements are present and in order
    let skip_list_keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    let expected_keys: Vec<_> = expected.iter().copied().collect();
    assert_eq!(skip_list_keys, expected_keys);

    // Remove half the elements
    let to_remove: Vec<_> = expected.iter().step_by(2).copied().collect();
    for key in &to_remove {
        assert_eq!(skip_list.remove(key), Some(key * 2));
        expected.remove(key);
    }

    assert_eq!(skip_list.len(), expected.len());

    // Verify remaining elements
    let remaining_keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    let expected_remaining: Vec<_> = expected.iter().copied().collect();
    assert_eq!(remaining_keys, expected_remaining);
}

#[test]
fn test_string_keys() {
    let mut skip_list = SkipList::new();
    
    let words = ["apple", "banana", "cherry", "date", "elderberry"];
    for &word in &words {
        skip_list.insert(word.to_string(), word.len());
    }

    // Should be in alphabetical order
    let collected: Vec<_> = (&skip_list).into_iter().map(|(k, &v)| (k.clone(), v)).collect();
    assert_eq!(collected, vec![
        ("apple".to_string(), 5),
        ("banana".to_string(), 6),
        ("cherry".to_string(), 6),
        ("date".to_string(), 4),
        ("elderberry".to_string(), 10),
    ]);
}

#[test]
fn test_negative_numbers() {
    let mut skip_list = SkipList::new();
    
    let numbers = [-5i32, -1, 0, 3, -10, 7, -3];
    for &num in &numbers {
        skip_list.insert(num, num.abs());
    }

    // Should be in numerical order
    let keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    assert_eq!(keys, vec![-10, -5, -3, -1, 0, 3, 7]);
}