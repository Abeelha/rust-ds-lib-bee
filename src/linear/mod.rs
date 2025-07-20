//! Linear data structures with sequential element access patterns

pub mod stack;
pub mod queue;
pub mod linked_list;

// Re-export main types
pub use stack::Stack;
pub use queue::Queue;
pub use linked_list::LinkedList;