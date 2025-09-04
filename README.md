# SkipList - Rust Implementation

**⚠️ Educational Purpose Only**: This project is for learning and educational purposes only. It is not intended for production use.

A Skip List data structure implementation in Rust demonstrating probabilistic data structures and memory management concepts.

## Key Features

- **O(log n) Time Complexity**: Lookup, insertion, and removal operations

- **Span-based Indexing**: Each forward pointer maintains span information for O(log n) positional access
- **Random Access Optimization**: Efficient `index()` method using span traversal from highest to lowest level

## Basic Usage

```rust
use skiplist::SkipList;

let mut skip_list = SkipList::new();

// Insert key-value pairs
skip_list.insert("apple", 1);
skip_list.insert("banana", 2);
skip_list.insert("cherry", 3);

// Retrieve values
assert_eq!(skip_list.get(&"banana"), Some(&2));

// Remove values
assert_eq!(skip_list.remove(&"banana"), Some(2));

// Access by position (O(log n) time complexity)
assert_eq!(skip_list.index(0), Some((&"apple", &1)));
assert_eq!(skip_list.index(1), Some((&"cherry", &3)));

// Mutable access by position
if let Some((key, value)) = skip_list.index_mut(0) {
    *value = 10;
}

// Iterate in sorted order
for (key, value) in &skip_list {
    println!("{}: {}", key, value);
}
// Output:
// apple: 10
// cherry: 3
```

## Span Design & Random Access

This SkipList implementation features an innovative **span-based indexing system** that enables efficient random access:

### How Spans Work

Each forward pointer in the skip list maintains a "span" value indicating how many nodes it skips:

```
L2 |HEAD---------------------------------- -> (3: 3)------------ -> TAIL
   |(4)                                       (2)
   ||                                         |                     |
L1 |HEAD -> (-10: -10)-------------------- -> (3: 3)------------ -> TAIL
   |(1)     (3)                               (2)
   ||       |                                 |                     |
L0 |HEAD -> (-10: -10) -> (1: 1) -> (2: 2) -> (3: 3) -> (10: 10) -> TAIL
   |(1)     (1)           (1)       (1)       (1)       (1)
```

### Index Method Algorithm

The `index(i)` method achieves **O(log n) time complexity** by:

1. **Top-down traversal**: Start from the highest level
2. **Span-guided skipping**: Use span values to efficiently jump over nodes
3. **Level descent**: Drop to lower levels when span would overshoot target
4. **Precise positioning**: Locate exact position without linear traversal

```rust
// O(log n) positional access - much faster than O(n) iteration!
let element = skip_list.index(500);  // Direct access to 500th element
```

### Performance Benefits

- **Random Access**: O(log n) instead of O(n) for positional queries
- **Skip Optimization**: Spans enable intelligent level traversal

## Performance Comparison

Tested on MacBook Pro with M4 Pro.

Lookup performance comparison (10,000 elements):

| Data Structure | Average Total Lookup Time | Relative Performance |
| -------------- | ------------------------- | -------------------- |
| BTreeMap       | ~310µs                    | 1.0x (baseline)      |
| SkipList       | ~921µs                    | 2.9x slower          |
| LinkedList     | ~126ms                    | **406x slower**      |

![comparison](comparison.png)

**Key Insights:**

- SkipList provides O(log n) lookup performance, competitive with BTreeMap
- **Index method**: O(log n) random access using span information - significantly faster than linear traversal
- LinkedList requires O(n) linear search, making it impractical for lookups in large datasets
- SkipList offers a good balance between search efficiency and implementation simplicity

## Time Complexity Summary

| Operation            | Time Complexity       | Notes                            |
| -------------------- | --------------------- | -------------------------------- |
| `insert(key, value)` | O(log n) expected     | Probabilistic balancing          |
| `get(key)`           | O(log n) expected     | Key-based lookup                 |
| `remove(key)`        | O(log n) expected     | Key-based removal                |
| `index(i)`           | **O(log n) expected** | **Span-based positional access** |
| `len()`              | O(1)                  | Cached length                    |
| Iteration            | O(n)                  | Linear traversal at level 0      |

Run the benchmark yourself:

## Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## License

MIT License - for educational use only.
