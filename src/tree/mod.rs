//! Tree-based data structures

pub mod bst;
pub mod avl;
pub mod red_black;
pub mod trie;

pub use bst::BinarySearchTree;
pub use avl::AvlTree;
pub use red_black::RedBlackTree;
pub use trie::Trie;