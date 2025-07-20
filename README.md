# 🐝 Rust Data Structures Library (rust-ds-lib-bee)

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
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
  - AVL Tree with automatic balancing
  - Trie (prefix tree) for string operations
- **Hash Structures**
  - HashMap with separate chaining collision resolution
  - HashSet with set operations (union, intersection, difference)
- **Heap Structures**
  - Binary Heap (Min/Max variants)
  - Priority Queue with custom priorities
- **Graph Structures**
  - Graph with adjacency list representation
  - BFS, DFS, and pathfinding algorithms

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-ds-lib-bee = "0.1.0"
```

### Basic Usage

```rust
use rust_ds_lib_bee::{Stack, Queue, LinkedList, BinarySearchTree, HashMap, HashSet};

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

Contributions are welcome! Please see our [Contributing Guide](docs/contributing.md) for details.

### Development Setup

```bash
git clone https://github.com/yourusername/rust-ds-lib-bee.git
cd rust-ds-lib-bee
cargo test
cargo bench
```

## 📊 Performance

All data structures include comprehensive benchmarks. Run `cargo bench` to see performance characteristics on your system.

## 🛡️ Safety

This library prioritizes memory safety:
- Zero `unsafe` code in core implementations (unless performance-critical and well-documented)
- Comprehensive test coverage including edge cases
- Property-based testing for mathematical invariants

## 📋 Minimum Supported Rust Version (MSRV)

Rust 1.70 or later.

## 📄 License

Licensed under either of
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


## 🙏 Acknowledgments

This project is inspired by educational resources and the Rust community's commitment to safe systems programming.

---

**Educational Note**: This library serves as both a practical tool and a learning resource for understanding data structures and algorithms in Rust. Each implementation includes detailed documentation explaining the design decisions and trade-offs involved.
