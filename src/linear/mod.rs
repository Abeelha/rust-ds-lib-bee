//! Linear data structures with sequential element access patterns

pub mod linked_list;
pub mod queue;
pub mod stack;

// Re-export main types
pub use linked_list::LinkedList;
pub use queue::Queue;
pub use stack::Stack;
