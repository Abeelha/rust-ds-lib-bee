use crate::heap::BinaryHeap;
use crate::utils::{Clear, Size, Peek};
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
struct PriorityItem<T, P> {
    item: T,
    priority: P,
}

impl<T, P: Ord> PartialEq for PriorityItem<T, P> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T, P: Ord> Eq for PriorityItem<T, P> {}

impl<T, P: Ord> PartialOrd for PriorityItem<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, P: Ord> Ord for PriorityItem<T, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub struct PriorityQueue<T, P> {
    heap: BinaryHeap<PriorityItem<T, P>>,
}

impl<T, P: Ord> PriorityQueue<T, P> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::max_heap(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }


    pub fn push(&mut self, item: T, priority: P) {
        self.heap.push(PriorityItem { item, priority });
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|priority_item| priority_item.item)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|priority_item| &priority_item.item)
    }

    pub fn peek_priority(&self) -> Option<&P> {
        self.heap.peek().map(|priority_item| &priority_item.priority)
    }

    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &P)> {
        self.heap.iter().map(|item| (&item.item, &item.priority))
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|priority_item| priority_item.item)
            .collect()
    }
}

impl<T, P: Ord> Default for PriorityQueue<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, P> Clear for PriorityQueue<T, P> {
    fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T, P> Size for PriorityQueue<T, P> {
    fn len(&self) -> usize {
        self.heap.len()
    }
}

impl<T, P: Ord> Peek<T> for PriorityQueue<T, P> {
    fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|priority_item| &priority_item.item)
    }
}

impl<T: fmt::Debug, P: fmt::Debug + Ord> fmt::Debug for PriorityQueue<T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PriorityQueue")
            .field("heap", &self.heap)
            .finish()
    }
}

impl<T, P: Ord> FromIterator<(T, P)> for PriorityQueue<T, P> {
    fn from_iter<I: IntoIterator<Item = (T, P)>>(iter: I) -> Self {
        let mut queue = PriorityQueue::new();
        for (item, priority) in iter {
            queue.push(item, priority);
        }
        queue
    }
}

impl<T, P: Ord> Extend<(T, P)> for PriorityQueue<T, P> {
    fn extend<I: IntoIterator<Item = (T, P)>>(&mut self, iter: I) {
        for (item, priority) in iter {
            self.push(item, priority);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue_is_empty() {
        let queue: PriorityQueue<i32, i32> = PriorityQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn priority_ordering() {
        let mut queue = PriorityQueue::new();

        queue.push("low", 1);
        queue.push("high", 10);
        queue.push("medium", 5);

        assert_eq!(queue.pop(), Some("high"));
        assert_eq!(queue.pop(), Some("medium"));
        assert_eq!(queue.pop(), Some("low"));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn peek_operations() {
        let mut queue = PriorityQueue::new();
        queue.push("task", 5);
        queue.push("urgent", 10);

        assert_eq!(queue.peek(), Some(&"urgent"));
        assert_eq!(queue.peek_priority(), Some(&10));
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn capacity_management() {
        let queue: PriorityQueue<i32, i32> = PriorityQueue::with_capacity(10);
        assert_eq!(queue.capacity(), 10);
        assert!(queue.is_empty());
    }

    #[test]
    fn iter_functionality() {
        let mut queue = PriorityQueue::new();
        queue.push("a", 1);
        queue.push("b", 2);
        queue.push("c", 3);

        let items: Vec<_> = queue.iter().collect();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn into_sorted_vec() {
        let mut queue = PriorityQueue::new();
        queue.push("c", 3);
        queue.push("a", 1);
        queue.push("b", 2);

        let sorted = queue.into_sorted_vec();
        assert_eq!(sorted, vec!["c", "b", "a"]);
    }

    #[test]
    fn from_iterator() {
        let items = vec![("low", 1), ("high", 10), ("medium", 5)];
        let queue: PriorityQueue<_, _> = items.into_iter().collect();

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.peek(), Some(&"high"));
    }

    #[test]
    fn clear_queue() {
        let mut queue = PriorityQueue::new();
        queue.push("item", 1);
        queue.push("another", 2);

        assert!(!queue.is_empty());
        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn equal_priorities() {
        let mut queue = PriorityQueue::new();
        queue.push("first", 5);
        queue.push("second", 5);
        queue.push("third", 5);

        assert_eq!(queue.len(), 3);

        let first = queue.pop().unwrap();
        let second = queue.pop().unwrap();
        let third = queue.pop().unwrap();

        assert!(["first", "second", "third"].contains(&first));
        assert!(["first", "second", "third"].contains(&second));
        assert!(["first", "second", "third"].contains(&third));
    }
}