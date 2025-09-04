use std::{borrow::Borrow, fmt, mem::MaybeUninit, ptr::NonNull};

mod iter;

pub trait Key: Ord + fmt::Debug {}

impl<T> Key for T where T: Ord + fmt::Debug {}

pub trait Value: fmt::Debug {}
impl<T> Value for T where T: fmt::Debug {}

pub struct Node<K, V> {
    key: MaybeUninit<K>,
    value: MaybeUninit<V>,
    forward: Vec<ForwardPtr<K, V>>,
    level: usize,
}

impl<K: Key, V: Value> Node<K, V> {
    pub fn key(&self) -> &K {
        unsafe { self.key.assume_init_ref() }
    }

    pub fn key_mut(&mut self) -> &mut K {
        unsafe { self.key.assume_init_mut() }
    }

    pub fn value(&self) -> &V {
        unsafe { self.value.assume_init_ref() }
    }

    pub fn value_mut(&mut self) -> &mut V {
        unsafe { self.value.assume_init_mut() }
    }
}

impl<K: Key + fmt::Display, V: Value + fmt::Display> fmt::Display for Node<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}: {})", self.key(), self.value())
    }
}

type NodePtr<K, V> = NonNull<Node<K, V>>;

#[derive(Debug)]
struct ForwardPtr<K, V> {
    ptr: NodePtr<K, V>,
    span: usize,
}

impl<K, V> Clone for ForwardPtr<K, V> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            span: self.span,
        }
    }
}

impl<K, V> Copy for ForwardPtr<K, V> {}

impl<K, V> PartialEq for ForwardPtr<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<K, V> Default for ForwardPtr<K, V> {
    fn default() -> Self {
        Self {
            ptr: NonNull::dangling(),
            span: 0,
        }
    }
}

#[derive(Debug)]
pub struct SkipList<K: Key, V: Value> {
    head: NodePtr<K, V>,
    tail: NodePtr<K, V>,
    level: usize,
    len: usize,
}

const MAX_LEVEL: usize = 32;

impl<K: Key, V: Value> SkipList<K, V> {
    pub fn new() -> Self {
        let tail: Box<Node<_, _>> = Box::new(Node {
            key: MaybeUninit::uninit(),
            value: MaybeUninit::uninit(),
            forward: vec![],
            level: 0,
        });

        let tail_ptr = NonNull::from(Box::leak(tail));

        let head = Box::new(Node {
            key: MaybeUninit::uninit(),
            value: MaybeUninit::uninit(),
            forward: vec![ForwardPtr {
                ptr: tail_ptr,
                span: 1,
            }],
            level: 0,
        });

        let head_ptr = NonNull::from(Box::leak(head));

        Self {
            head: head_ptr,
            tail: tail_ptr,
            level: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn verify_spans(&self) -> bool {
        // First, traverse level 0 to build a position index for each node
        let mut node_positions = std::collections::HashMap::new();
        let mut current = self.head;
        let mut position = 0;
        
        // Map each node pointer to its position in level 0
        while !self.is_tail(current) {
            node_positions.insert(current, position);
            current = unsafe { current.as_ref() }.forward[0].ptr;
            position += 1;
        }
        node_positions.insert(self.tail, position); // tail position
        
        // Now verify spans at each level
        for level in 0..=self.level {
            let mut current = self.head;
            
            while !self.is_tail(current) {
                let node_ref = unsafe { current.as_ref() };
                
                // Skip if this node doesn't have this level
                if level >= node_ref.forward.len() {
                    break;
                }
                
                let forward_ptr = node_ref.forward[level];
                let next_node = forward_ptr.ptr;
                let span = forward_ptr.span;

                // Get positions from our index
                let current_pos = node_positions[&current];
                let next_pos = node_positions[&next_node];
                
                // Span should equal the difference in positions
                let expected_span = next_pos - current_pos;
                if span != expected_span {
                    return false;
                }
                
                current = next_node;
            }
        }
        
        true
    }

    fn is_head(&self, node: NodePtr<K, V>) -> bool {
        node == self.head
    }

    fn is_tail(&self, node: NodePtr<K, V>) -> bool {
        node == self.tail
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let level = Self::rand_level();

        if level > self.level {
            for _ in (self.level + 1)..=level {
                unsafe {
                    self.head.as_mut().forward.push(ForwardPtr {
                        ptr: self.tail,
                        span: self.len + 1,
                    });
                }
            }
            self.level = level;
        }

        let mut update = vec![NodePtr::dangling(); self.level + 1];
        let mut steps = vec![0; self.level + 1];
        let mut step = 0;

        let mut cur = self.head;
        for i in (0..=self.level).rev() {
            loop {
                let cur_node_ref = unsafe { cur.as_ref() };
                let next = cur_node_ref.forward[i].ptr;

                if self.is_tail(next) {
                    break;
                }
                let next_key = (unsafe { next.as_ref() }).key();
                if next_key < &key {
                    step += cur_node_ref.forward[i].span;
                    cur = next;
                } else {
                    break;
                }
            }
            update[i] = cur;
            steps[i] = step;
        }

        let mut next = unsafe { cur.as_ref() }.forward[0].ptr;

        if !self.is_tail(next) && unsafe { next.as_ref() }.key() == &key {
            // already exists, replace value
            let old_v = std::mem::replace(unsafe { next.as_mut() }.value_mut(), value);

            return Some(old_v);
        }

        // cur = next;

        step += 1;

        let mut forward = vec![ForwardPtr::default(); level + 1];

        let new_node = Box::new(Node {
            key: MaybeUninit::new(key),
            value: MaybeUninit::new(value),
            forward: vec![],
            level,
        });

        let mut new_node_ptr = NonNull::from(Box::leak(new_node));

        for i in (0..=self.level).rev() {
            let update_node = unsafe { update[i].as_mut() };
            if i <= level {
                let cur_span = step - steps[i];

                // println!(
                //     "leve: {i}, step i: {}, updated_node span:{} ,step: {step}",
                //     steps[i], update_node.forward[i].span
                // );
                forward[i] = ForwardPtr {
                    ptr: update_node.forward[i].ptr,
                    span: steps[i] + update_node.forward[i].span - step + 1,
                };
                // println!(
                //     "span: {}",
                //     steps[i] + update_node.forward[i].span - cur_span
                // );

                update_node.forward[i].ptr = new_node_ptr;
                update_node.forward[i].span = cur_span;
            } else {
                update_node.forward[i].span += 1;
            }
        }

        //
        // 1 2 3    (5)   7
        // 1 2 3 4  (5) 6 7

        unsafe { new_node_ptr.as_mut() }.forward = forward;

        self.len += 1;
        None
    }

    pub fn remove<Q: fmt::Debug>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        // println!("removing key: {key:?}");
        let mut update = vec![NonNull::dangling(); self.level + 1];

        let mut cur = self.head;
        for i in (0..=self.level).rev() {
            loop {
                let cur_node_ref = unsafe { cur.as_ref() };
                if self.is_tail(cur_node_ref.forward[i].ptr) {
                    break;
                }

                let next_ptr = cur_node_ref.forward[i].ptr;
                let next_key = unsafe { next_ptr.as_ref() }.key();
                if next_key.borrow() < key {
                    cur = next_ptr;
                } else {
                    break;
                }
            }
            update[i] = cur;
        }

        let next = unsafe { cur.as_ref() }.forward[0].ptr;
        if self.is_tail(next) {
            return None;
        }
        cur = next;

        if unsafe { cur.as_ref() }.key().borrow() != key {
            return None;
        }

        let to_remove = cur;

        for i in (0..=self.level).rev() {
            let update_node = unsafe { update[i].as_mut() };

            unsafe {
                if i <= to_remove.as_ref().level {
                    update_node.forward[i] = ForwardPtr {
                        ptr: cur.as_ref().forward[i].ptr,
                        span: update[i].as_ref().forward[i].span
                            + to_remove.as_ref().forward[i].span
                            - 1,
                    };
                } else {
                    update_node.forward[i].span -= 1;
                }
            }
        }

        // println!("after remove, before clean level:\n {}", self);

        let mut level_down = 0;
        for i in (0..=self.level).rev() {
            // println!("start cleaning at level {i}");
            let head_next = unsafe { self.head.as_ref().forward[i].ptr };

            if self.is_tail(head_next) && i > 0 {
                // println!("level down once at: {i}");
                level_down += 1;
                unsafe { self.head.as_mut() }.forward.pop();
            } else {
                break;
            }
        }

        self.level -= level_down;

        self.len -= 1;

        let node = unsafe { Box::from_raw(cur.as_ptr()) };
        Some(unsafe { node.value.assume_init() })
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut cur = self.head;
        for i in (0..=self.level).rev() {
            loop {
                let next = unsafe { cur.as_ref() }.forward[i].ptr;
                if self.is_tail(next) {
                    break;
                }
                let next_key = (unsafe { next.as_ref() }).key();

                if next_key.borrow() == key {
                    return Some(unsafe { next.as_ref().value() });
                }

                if next_key.borrow() < key {
                    cur = next;
                } else {
                    break;
                }
            }
        }

        None
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut cur = self.head;
        for i in (0..=self.level).rev() {
            loop {
                let mut next = unsafe { cur.as_ref() }.forward[i].ptr;
                if self.is_tail(next) {
                    break;
                }
                let next_key = (unsafe { next.as_ref() }).key();

                if next_key.borrow() == key {
                    return Some(unsafe { next.as_mut().value_mut() });
                }

                if next_key.borrow() < key {
                    cur = next;
                } else {
                    break;
                }
            }
        }

        None
    }

    /// Get the key-value pair at the specified index using span information for efficient traversal.
    /// Returns None if the index is out of bounds.
    /// 
    /// Time complexity: O(log n) expected
    /// 
    /// # Examples
    /// 
    /// ```
    /// use skiplist::SkipList;
    /// 
    /// let mut skip_list = SkipList::new();
    /// skip_list.insert(1, "first");
    /// skip_list.insert(2, "second");
    /// skip_list.insert(3, "third");
    /// 
    /// assert_eq!(skip_list.index(0), Some((&1, &"first")));
    /// assert_eq!(skip_list.index(1), Some((&2, &"second")));
    /// assert_eq!(skip_list.index(2), Some((&3, &"third")));
    /// assert_eq!(skip_list.index(3), None);
    /// ```
    pub fn index(&self, index: usize) -> Option<(&K, &V)> {
        if index >= self.len {
            return None;
        }

        let target_index = index + 1; // +1 because head is at position 0
        let mut current = self.head;
        let mut current_index = 0;

        // Traverse from highest level to lowest, using spans to skip efficiently
        for level in (0..=self.level).rev() {
            while current_index < target_index {
                let current_node = unsafe { current.as_ref() };
                
                // Check if this level has a forward pointer
                if level < current_node.forward.len() {
                    let forward_ptr = current_node.forward[level];
                    let next_index = current_index + forward_ptr.span;
                    
                    // If the next span would overshoot our target, move to lower level
                    if next_index > target_index {
                        break;
                    }
                    
                    // If we reach the tail, we've gone too far
                    if self.is_tail(forward_ptr.ptr) {
                        break;
                    }
                    
                    current = forward_ptr.ptr;
                    current_index = next_index;
                } else {
                    break;
                }
            }
        }

        // If we found the exact index and it's not the head or tail
        if current_index == target_index && !self.is_head(current) && !self.is_tail(current) {
            let node = unsafe { current.as_ref() };
            Some((node.key(), node.value()))
        } else {
            None
        }
    }

    /// Get a mutable reference to the value at the specified index.
    /// Returns None if the index is out of bounds.
    /// 
    /// Time complexity: O(log n) expected
    pub fn index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        if index >= self.len {
            return None;
        }

        let target_index = index + 1; // +1 because head is at position 0
        let mut current = self.head;
        let mut current_index = 0;

        // Traverse from highest level to lowest, using spans to skip efficiently
        for level in (0..=self.level).rev() {
            while current_index < target_index {
                let current_node = unsafe { current.as_ref() };
                
                // Check if this level has a forward pointer
                if level < current_node.forward.len() {
                    let forward_ptr = current_node.forward[level];
                    let next_index = current_index + forward_ptr.span;
                    
                    // If the next span would overshoot our target, move to lower level
                    if next_index > target_index {
                        break;
                    }
                    
                    // If we reach the tail, we've gone too far
                    if self.is_tail(forward_ptr.ptr) {
                        break;
                    }
                    
                    current = forward_ptr.ptr;
                    current_index = next_index;
                } else {
                    break;
                }
            }
        }

        // If we found the exact index and it's not the head or tail
        if current_index == target_index && !self.is_head(current) && !self.is_tail(current) {
            let node = unsafe { current.as_mut() };
            let key_ref = unsafe { current.as_ref() }.key(); // Get immutable reference to key
            let value_ref = node.value_mut(); // Get mutable reference to value
            Some((key_ref, value_ref))
        } else {
            None
        }
    }

    fn rand_level() -> usize {
        let mut level = 0;

        while rand::random::<f64>() < 0.5 && level < MAX_LEVEL {
            level += 1;
        }

        level
    }
}

impl<K: Key, V: Value> Drop for SkipList<K, V> {
    fn drop(&mut self) {
        unsafe {
            let mut cur = self.head.as_ref().forward[0].ptr;

            while !self.is_tail(cur) {
                let next = cur.as_ref().forward[0].ptr;
                let _ = Box::from_raw(cur.as_ptr());
                cur = next;
            }

            let _ = Box::from_raw(self.head.as_ptr());
            let _ = Box::from_raw(self.tail.as_ptr());
        }
    }
}

impl<K: Key + fmt::Debug, V: Value + fmt::Debug> fmt::Display for SkipList<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1. Get all nodes from level 0. This defines the columns of our graph.
        let mut nodes_l0 = vec![];
        let mut current = self.head;
        loop {
            nodes_l0.push(current);
            if self.is_tail(current) {
                break;
            }
            current = unsafe { current.as_ref() }.forward[0].ptr;
        }

        // 2. Get string representations for each node.
        let node_reprs: Vec<String> = nodes_l0
            .iter()
            .map(|&node_ptr| {
                if self.is_head(node_ptr) {
                    "HEAD".to_string()
                } else if self.is_tail(node_ptr) {
                    "TAIL".to_string()
                } else {
                    format!("({:?}: {:?})", unsafe { node_ptr.as_ref().key() }, unsafe {
                        node_ptr.as_ref().value()
                    })
                }
            })
            .collect();

        // 3. Print each level from top to bottom.
        for i in (0..=self.level).rev() {
            // Print node line
            write!(f, "L{:<2}|", i)?;
            let mut node_on_level = self.head;
            for (l0_idx, &l0_node) in nodes_l0.iter().enumerate() {
                let repr = &node_reprs[l0_idx];
                let is_node_on_level = l0_node == node_on_level;

                if l0_idx > 0 {
                    if is_node_on_level {
                        write!(f, " -> ")?;
                    } else {
                        write!(f, "----")?;
                    }
                }

                if is_node_on_level {
                    write!(f, "{}", repr)?;
                    if !self.is_tail(node_on_level) {
                        node_on_level = unsafe { node_on_level.as_ref() }.forward[i].ptr;
                    }
                } else {
                    write!(f, "{}", "-".repeat(repr.len()))?;
                }
            }
            writeln!(f)?;

            // Print spans line
            write!(f, "   |")?;
            let mut node_on_level_for_span = self.head;
            for (l0_idx, &l0_node) in nodes_l0.iter().enumerate() {
                let repr = &node_reprs[l0_idx];
                if l0_idx > 0 {
                    write!(f, "    ")?;
                }

                if l0_node == node_on_level_for_span {
                    if !self.is_tail(node_on_level_for_span) {
                        let forward_ptr = unsafe { node_on_level_for_span.as_ref() }.forward[i];
                        let span = forward_ptr.span;
                        let span_str = format!("({})", span);
                        write!(f, "{:<width$}", span_str, width = repr.len())?;
                        node_on_level_for_span = forward_ptr.ptr;
                    } else {
                        // TAIL node has no outgoing span
                        write!(f, "{}", " ".repeat(repr.len()))?;
                    }
                } else {
                    write!(f, "{}", " ".repeat(repr.len()))?;
                }
            }
            writeln!(f)?;

            // 4. Print vertical connectors.
            if i > 0 {
                write!(f, "   |")?;
                let mut node_on_level_for_vertical = self.head;
                for (l0_idx, &l0_node) in nodes_l0.iter().enumerate() {
                    let repr = &node_reprs[l0_idx];
                    if l0_idx > 0 {
                        write!(f, "    ")?;
                    }

                    if l0_node == node_on_level_for_vertical {
                        write!(f, "|")?;
                        write!(f, "{}", " ".repeat(repr.len() - 1))?;
                        if !self.is_tail(node_on_level_for_vertical) {
                            node_on_level_for_vertical =
                                unsafe { node_on_level_for_vertical.as_ref() }.forward[i].ptr;
                        }
                    } else {
                        write!(f, "{}", " ".repeat(repr.len()))?;
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_skiplist_operations() {
        let mut skip_list = SkipList::new();
        
        assert_eq!(skip_list.len(), 0);
        assert!(skip_list.is_empty());
        
        // Test insertion
        assert_eq!(skip_list.insert(5, "five"), None);
        assert_eq!(skip_list.len(), 1);
        assert!(!skip_list.is_empty());
        
        // Test retrieval
        assert_eq!(skip_list.get(&5), Some(&"five"));
        assert_eq!(skip_list.get(&3), None);
        
        // Test replacement
        assert_eq!(skip_list.insert(5, "FIVE"), Some("five"));
        assert_eq!(skip_list.len(), 1);
        assert_eq!(skip_list.get(&5), Some(&"FIVE"));
        
        // Test removal
        assert_eq!(skip_list.remove(&5), Some("FIVE"));
        assert_eq!(skip_list.len(), 0);
        assert!(skip_list.is_empty());
        assert_eq!(skip_list.get(&5), None);
    }
    
    #[test]
    fn test_ordering_property() {
        let mut skip_list = SkipList::new();
        let keys = [3, 1, 4, 1, 5, 9, 2, 6, 5];
        
        for &key in &keys {
            skip_list.insert(key, key * 10);
        }
        
        // Collect keys from iterator - should be in sorted order
        let result_keys: Vec<_> = (&skip_list).into_iter().map(|(&k, _)| k).collect();
        assert_eq!(result_keys, vec![1, 2, 3, 4, 5, 6, 9]);
    }
    
    #[test]
    fn test_span_verification() {
        let mut skip_list = SkipList::new();
        
        // Test empty list
        assert!(skip_list.verify_spans());
        
        // Test with elements
        for i in [10, 5, 15, 3, 7, 12, 18] {
            skip_list.insert(i, i);
            assert!(skip_list.verify_spans(), "Span verification failed after inserting {}", i);
        }
        
        // Test after removals
        for i in [5, 15, 3] {
            skip_list.remove(&i);
            assert!(skip_list.verify_spans(), "Span verification failed after removing {}", i);
        }
    }

    #[test]
    fn test_index_basic() {
        let mut skip_list = SkipList::new();
        
        // Test empty list
        assert_eq!(skip_list.index(0), None);
        
        // Test single element
        skip_list.insert(5, "five");
        assert_eq!(skip_list.index(0), Some((&5, &"five")));
        assert_eq!(skip_list.index(1), None);
        
        // Test multiple elements
        skip_list.insert(3, "three");
        skip_list.insert(7, "seven");
        skip_list.insert(1, "one");
        
        // Elements should be in sorted order by key
        assert_eq!(skip_list.index(0), Some((&1, &"one")));
        assert_eq!(skip_list.index(1), Some((&3, &"three")));
        assert_eq!(skip_list.index(2), Some((&5, &"five")));
        assert_eq!(skip_list.index(3), Some((&7, &"seven")));
        assert_eq!(skip_list.index(4), None);
    }

    #[test]
    fn test_index_mut() {
        let mut skip_list = SkipList::new();
        
        skip_list.insert(1, 10);
        skip_list.insert(2, 20);
        skip_list.insert(3, 30);
        
        // Test mutable access
        if let Some((key, value)) = skip_list.index_mut(1) {
            assert_eq!(*key, 2);
            *value = 200;
        }
        
        // Verify the change
        assert_eq!(skip_list.index(1), Some((&2, &200)));
        assert_eq!(skip_list.get(&2), Some(&200));
    }

    #[test]
    fn test_index_large_dataset() {
        let mut skip_list = SkipList::new();
        
        // Insert many elements
        let elements: Vec<i32> = (0..1000).collect();
        for &elem in &elements {
            skip_list.insert(elem, elem * 10);
        }
        
        // Test random indices
        for &i in &[0, 50, 100, 500, 999] {
            let result = skip_list.index(i);
            assert_eq!(result, Some((&elements[i], &(elements[i] * 10))));
        }
        
        // Test out of bounds
        assert_eq!(skip_list.index(1000), None);
        assert_eq!(skip_list.index(1500), None);
    }

    #[test]
    fn test_index_with_removals() {
        let mut skip_list = SkipList::new();
        
        // Insert elements 0..10
        for i in 0..10 {
            skip_list.insert(i, i * 10);
        }
        
        // Remove some elements [2, 4, 6, 8]
        for i in [2, 4, 6, 8] {
            skip_list.remove(&i);
        }
        
        // Remaining elements should be [0, 1, 3, 5, 7, 9]
        let expected = [0, 1, 3, 5, 7, 9];
        
        for (idx, &expected_key) in expected.iter().enumerate() {
            assert_eq!(skip_list.index(idx), Some((&expected_key, &(expected_key * 10))));
        }
        
        // Test out of bounds after removals
        assert_eq!(skip_list.index(6), None);
    }

    #[test]
    fn test_index_random_insertions() {
        let mut skip_list = SkipList::new();
        
        // Insert in random order
        let mut keys = vec![15, 3, 8, 1, 12, 20, 5, 18, 9, 7];
        for &key in &keys {
            skip_list.insert(key, key);
        }
        
        // Sort keys to get expected order
        keys.sort();
        
        // Test that index returns elements in sorted order
        for (idx, &expected_key) in keys.iter().enumerate() {
            assert_eq!(skip_list.index(idx), Some((&expected_key, &expected_key)));
        }
    }

    #[test]
    fn test_index_consistency_with_iteration() {
        let mut skip_list = SkipList::new();
        
        // Insert random elements
        let elements = [42, 17, 8, 23, 4, 15, 31, 6, 19, 12];
        for &elem in &elements {
            skip_list.insert(elem, elem * 2);
        }
        
        // Collect via iteration
        let iterated: Vec<_> = (&skip_list).into_iter().map(|(&k, &v)| (k, v)).collect();
        
        // Check that index method gives same results as iteration
        for (idx, &(expected_key, expected_value)) in iterated.iter().enumerate() {
            assert_eq!(skip_list.index(idx), Some((&expected_key, &expected_value)));
        }
    }
}
