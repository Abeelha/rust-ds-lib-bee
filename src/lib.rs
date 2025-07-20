#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Rust Data Structures Library
//!
//! A comprehensive, educational Rust library implementing fundamental and advanced data structures
//! with emphasis on safety, performance, and idiomatic Rust patterns.
//!
//! ## Features
//!
//! - **Linear Structures**: Stack, Queue, Deque, LinkedList, CircularBuffer
//! - **Tree Structures**: BST, AVL, Red-Black, Trie, B-Tree
//! - **Hash Structures**: HashMap, HashSet, BloomFilter
//! - **Heap Structures**: BinaryHeap, PriorityQueue
//! - **Graph Structures**: Adjacency List/Matrix, Graph Algorithms
//!
//! ## Usage
//!
//! ```rust
//! use rust_ds_lib_bee::linear::Stack;
//!
//! let mut stack = Stack::new();
//! stack.push(42);
//! assert_eq!(stack.pop(), Some(42));
//! ```

pub mod linear;
pub mod tree;
pub mod hash;
pub mod heap;
pub mod graph;
pub mod utils;

pub use linear::{Stack, Queue};
pub use tree::{BinarySearchTree, AvlTree, Trie};
pub use hash::{HashMap, HashSet};
pub use heap::{BinaryHeap, PriorityQueue};
pub use graph::Graph;
pub use utils::traits::*;
