//! Linked list implementation with dynamic memory allocation

use crate::utils::{Clear, Size};
use std::fmt;

/// A node in the linked list
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

/// A singly linked list
///
/// # Examples
///
/// ```rust
/// use rust_ds_lib_bee::linear::LinkedList;
///
/// let mut list = LinkedList::new();
/// list.push_front(1);
/// list.push_front(2);
/// assert_eq!(list.pop_front(), Some(2));
/// assert_eq!(list.pop_front(), Some(1));
/// ```
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    /// Creates a new empty linked list
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    /// Adds an element to the front of the list
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    /// Removes and returns the front element
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    /// Returns a reference to the front element without removing it
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    /// Returns a mutable reference to the front element
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    /// Returns an iterator over the list
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.as_deref(),
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clear for LinkedList<T> {
    fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Size for LinkedList<T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

/// An iterator over the elements of a LinkedList
pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

/// An owning iterator over the elements of a LinkedList
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
    }

    #[test]
    fn push_and_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&3));

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let collected: Vec<_> = list.iter().cloned().collect();
        assert_eq!(collected, vec![3, 2, 1]);
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let collected: Vec<_> = list.into_iter().collect();
        assert_eq!(collected, vec![3, 2, 1]);
    }
}
