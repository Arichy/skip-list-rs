use skiplist::SkipList;

#[test]
fn test_index_basic_operations() {
    let mut skip_list = SkipList::new();
    
    // Test empty list
    assert_eq!(skip_list.index(0), None);
    
    // Test single element
    skip_list.insert(42, "answer");
    assert_eq!(skip_list.index(0), Some((&42, &"answer")));
    assert_eq!(skip_list.index(1), None);
    
    // Test multiple elements
    skip_list.insert(10, "ten");
    skip_list.insert(30, "thirty");
    
    // Should be in sorted order
    assert_eq!(skip_list.index(0), Some((&10, &"ten")));
    assert_eq!(skip_list.index(1), Some((&30, &"thirty")));
    assert_eq!(skip_list.index(2), Some((&42, &"answer")));
    assert_eq!(skip_list.index(3), None);
}

#[test]
fn test_index_large_dataset() {
    let mut skip_list = SkipList::new();
    
    // Insert many elements
    for i in 0..1000 {
        skip_list.insert(i, i * 10);
    }
    
    // Test various indices
    for i in [0, 50, 500, 999] {
        assert_eq!(skip_list.index(i), Some((&i, &(i * 10))));
    }
    
    assert_eq!(skip_list.index(1000), None);
}

#[test]
fn test_index_with_removals() {
    let mut skip_list = SkipList::new();
    
    // Insert 0..10
    for i in 0..10 {
        skip_list.insert(i, i);
    }
    
    // Remove every other element
    for i in [0, 2, 4, 6, 8] {
        skip_list.remove(&i);
    }
    
    // Remaining: [1, 3, 5, 7, 9]
    let expected = [1, 3, 5, 7, 9];
    for (idx, &expected_key) in expected.iter().enumerate() {
        assert_eq!(skip_list.index(idx), Some((&expected_key, &expected_key)));
    }
    
    assert_eq!(skip_list.index(5), None);
}

#[test]
fn test_index_mut() {
    let mut skip_list = SkipList::new();
    
    for i in 0..5 {
        skip_list.insert(i, i * 10);
    }
    
    // Modify using index_mut
    if let Some((_, value)) = skip_list.index_mut(2) {
        *value = 999;
    }
    
    assert_eq!(skip_list.index(2), Some((&2, &999)));
}

#[test]
fn test_index_consistency_with_iteration() {
    let mut skip_list = SkipList::new();
    
    let elements = [42, 17, 8, 23, 4, 15, 31];
    for &elem in &elements {
        skip_list.insert(elem, elem * 2);
    }
    
    // Collect via iteration
    let iterated: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
    
    // Check index method matches iteration
    for (idx, &(expected_key, expected_value)) in iterated.iter().enumerate() {
        assert_eq!(skip_list.index(idx), Some((&expected_key, &expected_value)));
    }
}