//! Queue implementation with FIFO (First In, First Out) semantics

use crate::utils::{Clear, Size, Peek};

/// A queue data structure with FIFO semantics
///
/// # Examples
///
/// ```rust
/// use rust_ds_lib_bee::linear::Queue;
///
/// let mut queue = Queue::new();
/// queue.enqueue(1);
/// queue.enqueue(2);
/// assert_eq!(queue.dequeue(), Some(1));
/// assert_eq!(queue.dequeue(), Some(2));
/// assert_eq!(queue.dequeue(), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue<T> {
    data: Vec<T>,
    front: usize,
}

impl<T> Queue<T> {
    /// Creates a new empty queue
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            front: 0,
        }
    }

    /// Creates a new queue with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            front: 0,
        }
    }

    /// Adds an element to the back of the queue
    pub fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }

    /// Removes and returns the front element from the queue
    pub fn dequeue(&mut self) -> Option<T> {
        if self.front >= self.data.len() {
            self.clear();
            return None;
        }

        let result = Some(self.data.remove(self.front));
        
        // Compact the queue if we've removed too many elements
        if self.front > self.data.len() / 2 && self.front > 16 {
            self.data.drain(..self.front);
            self.front = 0;
        }

        result
    }

    /// Returns a reference to the front element without removing it
    pub fn front(&self) -> Option<&T> {
        if self.front < self.data.len() {
            Some(&self.data[self.front])
        } else {
            None
        }
    }

    /// Returns a reference to the back element without removing it
    pub fn back(&self) -> Option<&T> {
        if self.data.len() > self.front {
            self.data.last()
        } else {
            None
        }
    }

    /// Returns the current capacity of the queue
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clear for Queue<T> {
    fn clear(&mut self) {
        self.data.clear();
        self.front = 0;
    }
}

impl<T> Size for Queue<T> {
    fn len(&self) -> usize {
        if self.data.len() > self.front {
            self.data.len() - self.front
        } else {
            0
        }
    }
}

impl<T> Peek<T> for Queue<T> {
    fn peek(&self) -> Option<&T> {
        self.front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue_is_empty() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn enqueue_and_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn front_and_back() {
        let mut queue = Queue::new();
        assert_eq!(queue.front(), None);
        assert_eq!(queue.back(), None);

        queue.enqueue(1);
        assert_eq!(queue.front(), Some(&1));
        assert_eq!(queue.back(), Some(&1));

        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.front(), Some(&1));
        assert_eq!(queue.back(), Some(&3));

        queue.dequeue();
        assert_eq!(queue.front(), Some(&2));
        assert_eq!(queue.back(), Some(&3));
    }
}