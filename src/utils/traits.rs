//! Common traits and interfaces for data structures

/// A trait for containers that can be emptied
pub trait Clear {
    /// Removes all elements from the container
    fn clear(&mut self);
}

/// A trait for containers with a measurable size
pub trait Size {
    /// Returns the number of elements in the container
    fn len(&self) -> usize;

    /// Returns true if the container is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A trait for containers with a maximum capacity
pub trait Capacity: Size {
    /// Returns the maximum number of elements this container can hold
    fn capacity(&self) -> usize;

    /// Returns true if the container is at maximum capacity
    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }
}

/// A trait for containers that support peeking at elements without removing them
pub trait Peek<T> {
    /// Returns a reference to the element that would be returned by the next pop/dequeue operation
    fn peek(&self) -> Option<&T>;
}

/// A trait for containers that support mutable peeking
pub trait PeekMut<T> {
    /// Returns a mutable reference to the element that would be returned by the next pop/dequeue operation
    fn peek_mut(&mut self) -> Option<&mut T>;
}
