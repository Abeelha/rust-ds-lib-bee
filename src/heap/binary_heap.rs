use crate::utils::{Clear, Size, Peek, PeekMut};
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug)]
pub enum HeapType {
    Max,
    Min,
}

pub struct BinaryHeap<T> {
    data: Vec<T>,
    heap_type: HeapType,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        Self::max_heap()
    }

    pub fn max_heap() -> Self {
        Self {
            data: Vec::new(),
            heap_type: HeapType::Max,
        }
    }

    pub fn min_heap() -> Self {
        Self {
            data: Vec::new(),
            heap_type: HeapType::Min,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            heap_type: HeapType::Max,
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        let result = self.data.pop();
        
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        
        result
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn heap_type(&self) -> &HeapType {
        &self.heap_type
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent_idx = (idx - 1) / 2;
            if self.compare(idx, parent_idx) != Ordering::Greater {
                break;
            }
            self.data.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        loop {
            let left_child = 2 * idx + 1;
            let right_child = 2 * idx + 2;
            let mut largest = idx;

            if left_child < self.data.len() && self.compare(left_child, largest) == Ordering::Greater {
                largest = left_child;
            }

            if right_child < self.data.len() && self.compare(right_child, largest) == Ordering::Greater {
                largest = right_child;
            }

            if largest == idx {
                break;
            }

            self.data.swap(idx, largest);
            idx = largest;
        }
    }

    fn compare(&self, i: usize, j: usize) -> Ordering {
        match self.heap_type {
            HeapType::Max => self.data[i].cmp(&self.data[j]),
            HeapType::Min => self.data[j].cmp(&self.data[i]),
        }
    }

    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.data.len());
        while let Some(item) = self.pop() {
            result.push(item);
        }
        result
    }
}

impl<T: Ord> Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clear for BinaryHeap<T> {
    fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T> Size for BinaryHeap<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Peek<T> for BinaryHeap<T> {
    fn peek(&self) -> Option<&T> {
        self.data.first()
    }
}

impl<T> PeekMut<T> for BinaryHeap<T> {
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.first_mut()
    }
}

impl<T: Ord> FromIterator<T> for BinaryHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = BinaryHeap::new();
        for item in iter {
            heap.push(item);
        }
        heap
    }
}

impl<T: Ord> Extend<T> for BinaryHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for BinaryHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BinaryHeap")
            .field("data", &self.data)
            .field("heap_type", &self.heap_type)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_heap_is_empty() {
        let heap: BinaryHeap<i32> = BinaryHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        assert_eq!(heap.peek(), None);
    }

    #[test]
    fn max_heap_ordering() {
        let mut heap = BinaryHeap::max_heap();
        
        for i in [3, 1, 4, 1, 5, 9, 2, 6] {
            heap.push(i);
        }

        let mut result = Vec::new();
        while let Some(item) = heap.pop() {
            result.push(item);
        }

        assert_eq!(result, vec![9, 6, 5, 4, 3, 2, 1, 1]);
    }

    #[test]
    fn min_heap_ordering() {
        let mut heap = BinaryHeap::min_heap();
        
        for i in [3, 1, 4, 1, 5, 9, 2, 6] {
            heap.push(i);
        }

        let mut result = Vec::new();
        while let Some(item) = heap.pop() {
            result.push(item);
        }

        assert_eq!(result, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn peek_operations() {
        let mut heap = BinaryHeap::max_heap();
        heap.push(5);
        heap.push(3);
        heap.push(7);

        assert_eq!(heap.peek(), Some(&7));
        assert_eq!(heap.len(), 3);

        if let Some(top) = heap.peek_mut() {
            *top = 10;
        }
        assert_eq!(heap.peek(), Some(&10));
    }

    #[test]
    fn into_sorted_vec() {
        let mut heap = BinaryHeap::max_heap();
        for i in [3, 1, 4, 1, 5] {
            heap.push(i);
        }

        let sorted = heap.into_sorted_vec();
        assert_eq!(sorted, vec![5, 4, 3, 1, 1]);
    }

    #[test]
    fn from_iterator() {
        let values = vec![3, 1, 4, 1, 5, 9];
        let heap: BinaryHeap<_> = values.into_iter().collect();
        
        assert_eq!(heap.len(), 6);
        assert_eq!(heap.peek(), Some(&9));
    }

    #[test]
    fn capacity_management() {
        let heap: BinaryHeap<i32> = BinaryHeap::with_capacity(10);
        assert_eq!(heap.capacity(), 10);
        assert!(heap.is_empty());
    }

    #[test]
    fn heap_property_maintained() {
        let mut heap = BinaryHeap::max_heap();
        
        for i in 1..=20 {
            heap.push(i);
            assert_eq!(heap.peek(), Some(&i));
        }

        for expected in (1..=20).rev() {
            assert_eq!(heap.pop(), Some(expected));
        }
    }

    #[test]
    fn clear_heap() {
        let mut heap = BinaryHeap::max_heap();
        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert!(!heap.is_empty());
        heap.clear();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
    }
}