//! Hash-based data structures

pub mod hashmap;
pub mod hashset;

// TODO: Implement additional hash structures
// pub mod bloom_filter;

// Re-export main types
pub use hashmap::HashMap;
pub use hashset::HashSet;