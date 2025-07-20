//! Binary Search Tree implementation with ordered operations

use crate::utils::{Clear, Size};
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}

/// A Binary Search Tree maintaining ordered data
///
/// # Examples
///
/// ```rust
/// use rust_ds_lib_bee::tree::BinarySearchTree;
///
/// let mut bst = BinarySearchTree::new();
/// bst.insert(5);
/// bst.insert(3);
/// bst.insert(7);
/// assert!(bst.contains(&5));
/// assert_eq!(bst.remove(&3), true);
/// ```
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    /// Creates a new empty binary search tree
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, data: T) -> bool {
        let inserted = Self::insert_recursive(&mut self.root, data);
        if inserted {
            self.size += 1;
        }
        inserted
    }

    fn insert_recursive(node: &mut Option<Box<Node<T>>>, data: T) -> bool {
        match node {
            None => {
                *node = Some(Box::new(Node::new(data)));
                true
            }
            Some(ref mut n) => match data.cmp(&n.data) {
                Ordering::Less => Self::insert_recursive(&mut n.left, data),
                Ordering::Greater => Self::insert_recursive(&mut n.right, data),
                Ordering::Equal => {
                    n.data = data;
                    false
                }
            },
        }
    }

    pub fn remove(&mut self, data: &T) -> bool {
        let removed = Self::remove_recursive(&mut self.root, data);
        if removed {
            self.size -= 1;
        }
        removed
    }

    fn remove_recursive(node: &mut Option<Box<Node<T>>>, data: &T) -> bool {
        match node {
            None => false,
            Some(ref mut n) => match data.cmp(&n.data) {
                Ordering::Less => Self::remove_recursive(&mut n.left, data),
                Ordering::Greater => Self::remove_recursive(&mut n.right, data),
                Ordering::Equal => {
                    *node = match (n.left.take(), n.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (Some(left), Some(right)) => {
                            // Find the in-order successor (leftmost node in right subtree)
                            let mut successor = right;
                            if successor.left.is_none() {
                                successor.left = Some(left);
                                Some(successor)
                            } else {
                                let min_data = Self::extract_min(&mut successor.left);
                                Some(Box::new(Node {
                                    data: min_data,
                                    left: Some(left),
                                    right: Some(successor),
                                }))
                            }
                        }
                    };
                    true
                }
            },
        }
    }

    fn extract_min(node: &mut Option<Box<Node<T>>>) -> T {
        match node {
            None => panic!("extract_min called on None"),
            Some(ref mut n) => {
                if n.left.is_none() {
                    let extracted = node.take().unwrap();
                    *node = extracted.right;
                    extracted.data
                } else {
                    Self::extract_min(&mut n.left)
                }
            }
        }
    }

    pub fn contains(&self, data: &T) -> bool {
        Self::contains_recursive(&self.root, data)
    }

    fn contains_recursive(node: &Option<Box<Node<T>>>, data: &T) -> bool {
        match node {
            None => false,
            Some(n) => match data.cmp(&n.data) {
                Ordering::Less => Self::contains_recursive(&n.left, data),
                Ordering::Greater => Self::contains_recursive(&n.right, data),
                Ordering::Equal => true,
            },
        }
    }

    pub fn min(&self) -> Option<&T> {
        Self::min_recursive(&self.root)
    }

    fn min_recursive(node: &Option<Box<Node<T>>>) -> Option<&T> {
        match node {
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(&n.data)
                } else {
                    Self::min_recursive(&n.left)
                }
            }
        }
    }

    pub fn max(&self) -> Option<&T> {
        Self::max_recursive(&self.root)
    }

    fn max_recursive(node: &Option<Box<Node<T>>>) -> Option<&T> {
        match node {
            None => None,
            Some(n) => {
                if n.right.is_none() {
                    Some(&n.data)
                } else {
                    Self::max_recursive(&n.right)
                }
            }
        }
    }

    pub fn height(&self) -> usize {
        Self::height_recursive(&self.root)
    }

    fn height_recursive(node: &Option<Box<Node<T>>>) -> usize {
        match node {
            None => 0,
            Some(n) => {
                1 + std::cmp::max(
                    Self::height_recursive(&n.left),
                    Self::height_recursive(&n.right),
                )
            }
        }
    }

    pub fn iter(&self) -> InOrderIter<T> {
        let mut stack = Vec::new();
        Self::push_left_spine(&self.root, &mut stack);
        InOrderIter { stack }
    }

    fn push_left_spine<'a>(mut node: &'a Option<Box<Node<T>>>, stack: &mut Vec<&'a Node<T>>) {
        while let Some(n) = node {
            stack.push(n);
            node = &n.left;
        }
    }
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clear for BinarySearchTree<T> {
    fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }
}

impl<T> Size for BinarySearchTree<T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T: fmt::Debug> fmt::Debug for BinarySearchTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BinarySearchTree")
            .field("root", &self.root)
            .field("size", &self.size)
            .finish()
    }
}

pub struct InOrderIter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T: Ord> Iterator for InOrderIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let result = &node.data;
            BinarySearchTree::push_left_spine(&node.right, &mut self.stack);
            Some(result)
        } else {
            None
        }
    }
}

impl<T: Ord> FromIterator<T> for BinarySearchTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = BinarySearchTree::new();
        for item in iter {
            tree.insert(item);
        }
        tree
    }
}

impl<T: Ord> Extend<T> for BinarySearchTree<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tree_is_empty() {
        let tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.height(), 0);
        assert_eq!(tree.min(), None);
        assert_eq!(tree.max(), None);
    }

    #[test]
    fn insert_and_contains() {
        let mut tree = BinarySearchTree::new();

        assert!(tree.insert(5));
        assert!(!tree.insert(5));
        assert!(tree.insert(3));
        assert!(tree.insert(7));
        assert!(tree.insert(1));
        assert!(tree.insert(9));

        assert_eq!(tree.len(), 5);
        assert!(tree.contains(&5));
        assert!(tree.contains(&3));
        assert!(tree.contains(&7));
        assert!(tree.contains(&1));
        assert!(tree.contains(&9));
        assert!(!tree.contains(&4));
        assert!(!tree.contains(&0));
    }

    #[test]
    fn min_and_max() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);

        assert_eq!(tree.min(), Some(&1));
        assert_eq!(tree.max(), Some(&9));
    }

    #[test]
    fn remove() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);

        assert!(tree.remove(&1));
        assert!(!tree.contains(&1));
        assert_eq!(tree.len(), 4);

        assert!(tree.remove(&7));
        assert!(!tree.contains(&7));
        assert_eq!(tree.len(), 3);

        assert!(tree.remove(&5));
        assert!(!tree.contains(&5));
        assert_eq!(tree.len(), 2);

        assert!(!tree.remove(&10));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn height() {
        let mut tree = BinarySearchTree::new();
        assert_eq!(tree.height(), 0);

        tree.insert(5);
        assert_eq!(tree.height(), 1);

        tree.insert(3);
        tree.insert(7);
        assert_eq!(tree.height(), 2);

        tree.insert(1);
        tree.insert(9);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn iter_in_order() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);

        let values: Vec<_> = tree.iter().cloned().collect();
        assert_eq!(values, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn from_iterator() {
        let values = vec![5, 3, 7, 1, 9];
        let tree: BinarySearchTree<_> = values.into_iter().collect();

        assert_eq!(tree.len(), 5);
        assert!(tree.contains(&5));
        assert!(tree.contains(&3));
        assert!(tree.contains(&7));
        assert!(tree.contains(&1));
        assert!(tree.contains(&9));
    }

    #[test]
    fn clear() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);

        assert!(!tree.is_empty());
        tree.clear();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(!tree.contains(&5));
    }
}
