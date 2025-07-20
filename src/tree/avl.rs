use crate::utils::{Clear, Size};
use std::cmp::{max, Ordering};
use std::fmt;

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    height: i32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |n| n.height);
        let right_height = self.right.as_ref().map_or(0, |n| n.height);
        self.height = 1 + max(left_height, right_height);
    }

    fn balance_factor(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |n| n.height);
        let right_height = self.right.as_ref().map_or(0, |n| n.height);
        left_height - right_height
    }
}

pub struct AvlTree<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord> AvlTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, data: T) -> bool {
        let (new_root, inserted) = Self::insert_recursive(self.root.take(), data);
        self.root = new_root;
        if inserted {
            self.size += 1;
        }
        inserted
    }

    fn insert_recursive(node: Option<Box<Node<T>>>, data: T) -> (Option<Box<Node<T>>>, bool) {
        match node {
            None => (Some(Box::new(Node::new(data))), true),
            Some(mut n) => {
                let inserted = match data.cmp(&n.data) {
                    Ordering::Less => {
                        let (left, ins) = Self::insert_recursive(n.left.take(), data);
                        n.left = left;
                        ins
                    }
                    Ordering::Greater => {
                        let (right, ins) = Self::insert_recursive(n.right.take(), data);
                        n.right = right;
                        ins
                    }
                    Ordering::Equal => {
                        n.data = data;
                        false
                    }
                };

                n.update_height();
                (Some(Self::balance(n)), inserted)
            }
        }
    }

    pub fn remove(&mut self, data: &T) -> bool {
        let (new_root, removed) = Self::remove_recursive(self.root.take(), data);
        self.root = new_root;
        if removed {
            self.size -= 1;
        }
        removed
    }

    fn remove_recursive(node: Option<Box<Node<T>>>, data: &T) -> (Option<Box<Node<T>>>, bool) {
        match node {
            None => (None, false),
            Some(mut n) => match data.cmp(&n.data) {
                Ordering::Less => {
                    let (left, removed) = Self::remove_recursive(n.left.take(), data);
                    n.left = left;
                    n.update_height();
                    (Some(Self::balance(n)), removed)
                }
                Ordering::Greater => {
                    let (right, removed) = Self::remove_recursive(n.right.take(), data);
                    n.right = right;
                    n.update_height();
                    (Some(Self::balance(n)), removed)
                }
                Ordering::Equal => {
                    let result = match (n.left.take(), n.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (Some(left), Some(right)) => {
                            let (mut successor, new_right) = Self::extract_min(right);
                            successor.left = Some(left);
                            successor.right = new_right;
                            successor.update_height();
                            Some(Self::balance(successor))
                        }
                    };
                    (result, true)
                }
            },
        }
    }

    fn extract_min(mut node: Box<Node<T>>) -> (Box<Node<T>>, Option<Box<Node<T>>>) {
        match node.left.take() {
            None => {
                let right = node.right.take();
                (node, right)
            },
            Some(left) => {
                let (min_node, new_left) = Self::extract_min(left);
                node.left = new_left;
                node.update_height();
                (min_node, Some(Self::balance(node)))
            }
        }
    }

    fn balance(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let balance = node.balance_factor();

        if balance > 1 {
            if let Some(ref left) = node.left {
                if left.balance_factor() < 0 {
                    node.left = Some(Self::rotate_left(node.left.take().unwrap()));
                }
            }
            Self::rotate_right(node)
        } else if balance < -1 {
            if let Some(ref right) = node.right {
                if right.balance_factor() > 0 {
                    node.right = Some(Self::rotate_right(node.right.take().unwrap()));
                }
            }
            Self::rotate_left(node)
        } else {
            node
        }
    }

    fn rotate_left(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        node.update_height();
        new_root.left = Some(node);
        new_root.update_height();
        new_root
    }

    fn rotate_right(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        node.update_height();
        new_root.right = Some(node);
        new_root.update_height();
        new_root
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
        self.root.as_ref().map_or(0, |n| n.height as usize)
    }

    pub fn is_balanced(&self) -> bool {
        Self::check_balanced(&self.root)
    }

    fn check_balanced(node: &Option<Box<Node<T>>>) -> bool {
        match node {
            None => true,
            Some(n) => {
                let balance = n.balance_factor().abs();
                balance <= 1 && Self::check_balanced(&n.left) && Self::check_balanced(&n.right)
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

impl<T: Ord> Default for AvlTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clear for AvlTree<T> {
    fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }
}

impl<T> Size for AvlTree<T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T: fmt::Debug> fmt::Debug for AvlTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AvlTree")
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
            AvlTree::push_left_spine(&node.right, &mut self.stack);
            Some(result)
        } else {
            None
        }
    }
}

impl<T: Ord> FromIterator<T> for AvlTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = AvlTree::new();
        for item in iter {
            tree.insert(item);
        }
        tree
    }
}

impl<T: Ord> Extend<T> for AvlTree<T> {
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
        let tree: AvlTree<i32> = AvlTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.height(), 0);
        assert!(tree.is_balanced());
    }

    #[test]
    fn insert_maintains_balance() {
        let mut tree = AvlTree::new();
        
        for i in 1..=7 {
            tree.insert(i);
            assert!(tree.is_balanced());
        }
        
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn right_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);
        
        assert!(tree.is_balanced());
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn left_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        
        assert!(tree.is_balanced());
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn left_right_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        
        assert!(tree.is_balanced());
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn right_left_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(1);
        tree.insert(3);
        tree.insert(2);
        
        assert!(tree.is_balanced());
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn contains_and_operations() {
        let mut tree = AvlTree::new();
        for i in [4, 2, 6, 1, 3, 5, 7] {
            tree.insert(i);
        }

        for i in 1..=7 {
            assert!(tree.contains(&i));
        }
        assert!(!tree.contains(&8));
        
        assert_eq!(tree.min(), Some(&1));
        assert_eq!(tree.max(), Some(&7));
    }

    #[test]
    fn remove_maintains_balance() {
        let mut tree = AvlTree::new();
        for i in 1..=7 {
            tree.insert(i);
        }

        assert!(tree.remove(&4));
        assert!(tree.is_balanced());
        assert!(!tree.contains(&4));
        assert_eq!(tree.len(), 6);

        assert!(tree.remove(&1));
        assert!(tree.is_balanced());
        assert_eq!(tree.len(), 5);

        assert!(!tree.remove(&10));
        assert_eq!(tree.len(), 5);
    }

    #[test]
    fn iter_in_order() {
        let mut tree = AvlTree::new();
        for i in [4, 2, 6, 1, 3, 5, 7] {
            tree.insert(i);
        }

        let values: Vec<_> = tree.iter().cloned().collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn stress_test() {
        let mut tree = AvlTree::new();
        
        for i in 0..100 {
            tree.insert(i);
            assert!(tree.is_balanced());
        }
        
        for i in (0..100).step_by(2) {
            tree.remove(&i);
            assert!(tree.is_balanced());
        }
        
        assert_eq!(tree.len(), 50);
    }
}