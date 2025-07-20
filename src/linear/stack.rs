//! Stack implementation with LIFO (Last In, First Out) semantics

use crate::utils::{Clear, Peek, PeekMut, Size};

/// A stack data structure with LIFO semantics
///
/// # Examples
///
/// ```rust
/// use rust_ds_lib_bee::linear::Stack;
///
/// let mut stack = Stack::new();
/// stack.push(1);
/// stack.push(2);
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.pop(), Some(1));
/// assert_eq!(stack.pop(), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Creates a new stack with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Pushes an element onto the top of the stack
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// Removes and returns the top element from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Returns the current capacity of the stack
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clear for Stack<T> {
    fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T> Size for Stack<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Peek<T> for Stack<T> {
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

impl<T> PeekMut<T> for Stack<T> {
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);

        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1); // peek doesn't remove

        stack.push(100);
        assert_eq!(stack.peek(), Some(&100));
    }
}
