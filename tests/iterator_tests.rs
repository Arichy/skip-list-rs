use skiplist::SkipList;

#[test]
fn test_borrowed_iterator() {
    let mut skip_list = SkipList::new();
    
    for i in [3, 1, 4, 1, 5, 9, 2, 6] {
        skip_list.insert(i, i * 10);
    }

    // Test borrowed iterator
    let items: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
    assert_eq!(items, vec![(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60), (9, 90)]);

    // Skip list should still be usable after borrowed iteration
    assert_eq!(skip_list.len(), 7);
    assert_eq!(skip_list.get(&3), Some(&30));
}

#[test]
fn test_consuming_iterator() {
    let mut skip_list = SkipList::new();
    
    for i in [3, 1, 4, 1, 5, 9, 2, 6] {
        skip_list.insert(i, i * 10);
    }

    // Test consuming iterator
    let items: Vec<_> = skip_list.into_iter().collect();
    assert_eq!(items, vec![(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60), (9, 90)]);

    // skip_list is now consumed and cannot be used
}

#[test]
fn test_partial_consuming_iteration() {
    let mut skip_list = SkipList::new();
    
    for i in 1..=10 {
        skip_list.insert(i, i * 10);
    }

    let mut iter = skip_list.into_iter();
    
    // Take only first 3 items
    assert_eq!(iter.next(), Some((1, 10)));
    assert_eq!(iter.next(), Some((2, 20)));
    assert_eq!(iter.next(), Some((3, 30)));
    
    // Drop the iterator early - should properly clean up remaining items
    drop(iter);
}

#[test]
fn test_empty_iterator() {
    let skip_list: SkipList<i32, i32> = SkipList::new();
    
    // Test borrowed iterator on empty list
    let items: Vec<_> = (&skip_list).into_iter().collect();
    assert_eq!(items, vec![]);

    // Test consuming iterator on empty list
    let items: Vec<_> = skip_list.into_iter().collect();
    assert_eq!(items, vec![]);
}

#[test]
fn test_single_element_iterator() {
    let mut skip_list = SkipList::new();
    skip_list.insert("key", "value");
    
    // Test borrowed iterator
    let items: Vec<_> = (&skip_list).into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    assert_eq!(items, vec![("key".to_string(), "value".to_string())]);

    // Test consuming iterator
    let items: Vec<_> = skip_list.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    assert_eq!(items, vec![("key".to_string(), "value".to_string())]);
}

#[test]
fn test_iterator_size_hint() {
    let mut skip_list = SkipList::new();
    
    for i in 1..=5 {
        skip_list.insert(i, i);
    }

    let iter = (&skip_list).into_iter();
    // Note: The current iterator implementation doesn't provide size_hint,
    // but we can test that it iterates the correct number of times
    assert_eq!(iter.count(), 5);
}

#[test]
fn test_iterator_with_duplicate_keys() {
    let mut skip_list = SkipList::new();
    
    // Insert with duplicates (should replace values)
    skip_list.insert(1, 10);
    skip_list.insert(2, 20);
    skip_list.insert(1, 100); // Replaces the first value
    skip_list.insert(3, 30);

    let items: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
    assert_eq!(items, vec![(1, 100), (2, 20), (3, 30)]);
}

#[test]
fn test_iterator_preserves_order() {
    let mut skip_list = SkipList::new();
    
    // Insert in reverse order
    for i in (1..=100).rev() {
        skip_list.insert(i, i * 2);
    }

    // Iterator should still return in sorted order
    let keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
    let expected: Vec<_> = (1..=100).collect();
    assert_eq!(keys, expected);
}

#[test]
fn test_iterator_with_complex_types() {
    let mut skip_list = SkipList::new();
    
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Person {
        name: String,
        age: u32,
    }

    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];

    for person in people.clone() {
        skip_list.insert(person.clone(), person.age);
    }

    let collected: Vec<_> = (&skip_list).into_iter().map(|(k, &v)| (k.clone(), v)).collect();
    
    // Should be sorted by name (Alice < Bob < Charlie)
    assert_eq!(collected.len(), 3);
    assert_eq!(collected[0].0.name, "Alice");
    assert_eq!(collected[1].0.name, "Bob");
    assert_eq!(collected[2].0.name, "Charlie");
}