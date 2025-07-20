# 🐝 Rust Data Structures Library (rust-ds-lib-bee)

[![CI](https://github.com/yourusername/rust-ds-lib-bee/workflows/CI/badge.svg)](https://github.com/yourusername/rust-ds-lib-bee/actions)
[![Crates.io](https://img.shields.io/crates/v/rust-ds-lib-bee.svg)](https://crates.io/crates/rust-ds-lib-bee)
[![Documentation](https://docs.rs/rust-ds-lib-bee/badge.svg)](https://docs.rs/rust-ds-lib-bee)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/yourusername/rust-ds-lib-bee#license)

A comprehensive, educational Rust library implementing fundamental and advanced data structures with emphasis on safety, performance, and idiomatic Rust patterns.

## 🎯 Features

### ✅ Currently Implemented
- **Linear Structures**
  - Stack (LIFO) with generic support
  - Queue (FIFO) with efficient operations
  - Singly Linked List with iterator support

### 🚧 In Development
- **Tree Structures**: BST, AVL, Red-Black, Trie, B-Tree
- **Hash Structures**: HashMap, HashSet, BloomFilter
- **Heap Structures**: BinaryHeap, PriorityQueue
- **Graph Structures**: Adjacency List/Matrix, Graph Algorithms

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-ds-lib-bee = "0.1.0"
```

### Basic Usage

```rust
use rust_ds_lib_bee::linear::{Stack, Queue, LinkedList};

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
```

## 📖 Documentation

Full API documentation is available on [docs.rs](https://docs.rs/rust-ds-lib-bee).

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

This project follows a phased development approach:

- **Phase 1** ✅: Foundation (Linear structures, testing framework)
- **Phase 2** 🚧: Core Trees & Hashing
- **Phase 3** 📋: Advanced Structures
- **Phase 4** 📋: Polish & Release

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

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

This project is inspired by educational resources and the Rust community's commitment to safe systems programming.

---

**Educational Note**: This library serves as both a practical tool and a learning resource for understanding data structures and algorithms in Rust. Each implementation includes detailed documentation explaining the design decisions and trade-offs involved.
