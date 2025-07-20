# 🐝 Rust Data Structures Library (rust-ds-lib-bee)

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.1.1-blue)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)

A comprehensive, educational Rust library implementing fundamental and advanced data structures with emphasis on safety, performance, and idiomatic Rust patterns.

## 🎯 Features

### ✅ Implemented Data Structures

- **Linear Structures**
  - Stack (LIFO) with generic support
  - Queue (FIFO) with efficient operations
  - Singly Linked List with iterator support
- **Tree Structures**
  - Binary Search Tree with O(log n) operations
  - AVL Tree with automatic balancing (guaranteed O(log n))
  - Red-Black Tree with guaranteed O(log n) operations
  - Trie (prefix tree) for string operations
- **Hash Structures**
  - HashMap with separate chaining collision resolution
  - HashSet with set operations (union, intersection, difference)
  - BloomFilter probabilistic data structure with configurable false positive rate
- **Heap Structures**
  - Binary Heap (Min/Max variants)
  - Priority Queue with custom priorities
- **Graph Structures**
  - Graph with adjacency list representation
  - WeightedGraph for algorithms requiring edge weights
  - BFS, DFS, shortest path, and Dijkstra's algorithm

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-ds-lib-bee = "0.1.0"
```

### Basic Usage

```rust
use rust_ds_lib_bee::{Stack, Queue, LinkedList, BinarySearchTree, HashMap, HashSet, BloomFilter, WeightedGraph};

// Stack operations
let mut stack = Stack::new();
stack.push(1);
stack.push(2);
assert_eq!(stack.pop(), Some(2));

// Queue operations
let mut queue = Queue::new();
queue.enqueue("hello");
queue.enqueue("world");
assert_eq!(queue.dequeue(), Some("hello"));

// LinkedList operations
let mut list = LinkedList::new();
list.push_front(42);
assert_eq!(list.front(), Some(&42));

// Binary Search Tree operations
let mut tree = BinarySearchTree::new();
tree.insert(5);
tree.insert(3);
tree.insert(7);
assert!(tree.contains(&5));

// HashMap operations
let mut map = HashMap::new();
map.insert("key", "value");
assert_eq!(map.get(&"key"), Some(&"value"));

// HashSet operations
let mut set = HashSet::new();
set.insert(1);
set.insert(2);
assert!(set.contains(&1));

// BloomFilter operations
let mut filter = BloomFilter::new(1000, 0.01); // 1000 expected items, 1% false positive rate
filter.insert(&"hello");
filter.insert(&"world");
assert!(filter.contains(&"hello")); // might be true (no false negatives)
assert!(!filter.contains(&"missing")); // definitely false or false positive

// WeightedGraph and Dijkstra's algorithm
let mut graph = WeightedGraph::directed();
graph.add_edge("A", "B", 4);
graph.add_edge("A", "C", 2);
graph.add_edge("B", "D", 3);
graph.add_edge("C", "D", 1);

use rust_ds_lib_bee::dijkstra_shortest_path;
let (distance, path) = dijkstra_shortest_path(&graph, &"A", &"D");
assert_eq!(distance, Some(3)); // A -> C -> D = 2 + 1 = 3
assert_eq!(path, Some(vec!["A", "C", "D"]));
```

## 📖 Documentation

Run `cargo doc --open` to view the full API documentation locally.

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

## 🎓 Educational Value

This library is designed with learning in mind:

- **Comprehensive Documentation**: Every public API includes examples and complexity analysis
- **Progressive Complexity**: Start with simple structures, advance to sophisticated algorithms
- **Idiomatic Rust**: Showcases ownership, borrowing, and lifetime patterns
- **Performance Focused**: Benchmarks and complexity analysis for all operations

## 🔧 Development Status

Development completed in three phases:

- **Phase 1** ✅: Foundation (Linear structures, testing framework)
- **Phase 2** ✅: Core Trees & Hashing (BST, HashMap, HashSet)
- **Phase 3** ✅: Advanced Structures (AVL, Heaps, Graphs, Trie)

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details.

### Development Setup

```bash
git clone https://github.com/abeelha/rust-ds-lib-bee.git
cd rust-ds-lib-bee
cargo test
cargo bench
```

## 📊 Performance & Complexity Analysis

All data structures include comprehensive benchmarks. Run `cargo bench` to see performance characteristics on your system.

### Time Complexity Overview

| Data Structure           | Insert       | Search/Contains  | Delete       | Peek/Access  | Space               |
| ------------------------ | ------------ | ---------------- | ------------ | ------------ | ------------------- |
| **Stack**                | O(1)         | -                | O(1)         | O(1)         | O(n)                |
| **Queue**                | O(1)         | -                | O(1)         | O(1)         | O(n)                |
| **LinkedList**           | O(1)\*       | O(n)             | O(1)\*       | O(1)         | O(n)                |
| **BinarySearchTree**     | O(log n)\*\* | O(log n)\*\*     | O(log n)\*\* | O(log n)\*\* | O(n)                |
| **AVL Tree**             | O(log n)     | O(log n)         | O(log n)     | O(log n)     | O(n)                |
| **Red-Black Tree**       | O(log n)     | O(log n)         | O(log n)     | O(log n)     | O(n)                |
| **Trie**                 | O(m)         | O(m)             | O(m)         | O(1)         | O(ALPHABET × N × M) |
| **HashMap**              | O(1)\*\*\*   | O(1)\*\*\*       | O(1)\*\*\*   | O(1)\*\*\*   | O(n)                |
| **HashSet**              | O(1)\*\*\*   | O(1)\*\*\*       | O(1)\*\*\*   | -            | O(n)                |
| **BloomFilter**          | O(k)         | O(k)             | -            | -            | O(m)                |
| **BinaryHeap**           | O(log n)     | -                | O(log n)     | O(1)         | O(n)                |
| **PriorityQueue**        | O(log n)     | -                | O(log n)     | O(1)         | O(n)                |
| **Graph**                | O(1)         | O(V + E)         | O(V + E)     | O(1)         | O(V + E)            |
| **WeightedGraph**        | O(1)         | O(V + E)         | O(V + E)     | O(1)         | O(V + E)            |
| **Dijkstra's Algorithm** | -            | O((V + E) log V) | -            | -            | O(V)                |

**Notes:**
- \* Front/back operations only; arbitrary position is O(n)
- \*\* Worst case O(n) for unbalanced trees; average case O(log n)
- \*\*\* Average case; worst case O(n) due to hash collisions
- `m` = string length, `k` = number of hash functions, `V` = vertices, `E` = edges

### Space-Time Trade-offs

- **AVL vs Red-Black Trees**: AVL trees are more strictly balanced (faster lookups) but require more rotations during insertion/deletion
- **HashMap vs Trie**: HashMap offers O(1) operations but Trie provides prefix operations and guaranteed O(m) complexity
- **BloomFilter**: Trades false positives for memory efficiency - uses only ~10 bits per element regardless of element size
- **BinaryHeap**: Excellent for priority operations but doesn't support arbitrary key updates efficiently

## 🛡️ Safety

This library prioritizes memory safety:
- Zero `unsafe` code in core implementations (unless performance-critical and well-documented)
- Comprehensive test coverage including edge cases
- Property-based testing for mathematical invariants

## 📋 Minimum Supported Rust Version (MSRV)

Rust 1.70 or later.

## 📄 License

- MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## 🙏 Acknowledgments

This project is inspired by educational resources and the Rust community's commitment to safe systems programming.

---

**Educational Note**: This library serves as both a practical tool and a learning resource for understanding data structures and algorithms in Rust. Each implementation includes detailed documentation explaining the design decisions and trade-offs involved.
