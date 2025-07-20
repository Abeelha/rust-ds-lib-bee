use crate::utils::{Clear, Size};
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    color: Color,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            color: Color::Red,
            left: None,
            right: None,
        }
    }

    fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    fn is_black(&self) -> bool {
        self.color == Color::Black
    }
}

pub struct RedBlackTree<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, data: T) -> bool {
        let (new_root, inserted) = Self::insert_recursive(self.root.take(), data);
        self.root = new_root;
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
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

                let balanced = Self::balance_after_insert(n);
                (Some(balanced), inserted)
            }
        }
    }

    fn balance_after_insert(mut node: Box<Node<T>>) -> Box<Node<T>> {
        if Self::is_red_optional(&node.right) && !Self::is_red_optional(&node.left) {
            node = Self::rotate_left(node);
        }

        if Self::is_red_optional(&node.left)
            && node
                .left
                .as_ref()
                .is_some_and(|left| Self::is_red_optional(&left.left))
        {
            node = Self::rotate_right(node);
        }

        if Self::is_red_optional(&node.left) && Self::is_red_optional(&node.right) {
            Self::flip_colors(&mut node);
        }

        node
    }

    fn rotate_left(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        new_root.color = node.color;
        node.color = Color::Red;
        new_root.left = Some(node);
        new_root
    }

    fn rotate_right(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        new_root.color = node.color;
        node.color = Color::Red;
        new_root.right = Some(node);
        new_root
    }

    fn flip_colors(node: &mut Box<Node<T>>) {
        node.color = Color::Red;
        if let Some(ref mut left) = node.left {
            left.color = Color::Black;
        }
        if let Some(ref mut right) = node.right {
            right.color = Color::Black;
        }
    }

    fn is_red_optional(node: &Option<Box<Node<T>>>) -> bool {
        node.as_ref().is_some_and(|n| n.is_red())
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

    pub fn is_valid_red_black_tree(&self) -> bool {
        self.root.as_ref().map_or(true, |root| {
            root.is_black() && Self::validate_red_black_properties(root).is_some()
        })
    }

    fn validate_red_black_properties(node: &Node<T>) -> Option<usize> {
        let left_black_height = match &node.left {
            None => Some(1),
            Some(left) => {
                if node.is_red() && left.is_red() {
                    return None;
                }
                Self::validate_red_black_properties(left)
            }
        };

        let right_black_height = match &node.right {
            None => Some(1),
            Some(right) => {
                if node.is_red() && right.is_red() {
                    return None;
                }
                Self::validate_red_black_properties(right)
            }
        };

        match (left_black_height, right_black_height) {
            (Some(left), Some(right)) if left == right => {
                Some(left + if node.is_black() { 1 } else { 0 })
            }
            _ => None,
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

impl<T: Ord> Default for RedBlackTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clear for RedBlackTree<T> {
    fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }
}

impl<T> Size for RedBlackTree<T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<T: fmt::Debug> fmt::Debug for RedBlackTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedBlackTree")
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
            RedBlackTree::push_left_spine(&node.right, &mut self.stack);
            Some(result)
        } else {
            None
        }
    }
}

impl<T: Ord> FromIterator<T> for RedBlackTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = RedBlackTree::new();
        for item in iter {
            tree.insert(item);
        }
        tree
    }
}

impl<T: Ord> Extend<T> for RedBlackTree<T> {
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
        let tree: RedBlackTree<i32> = RedBlackTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.height(), 0);
        assert!(tree.is_valid_red_black_tree());
    }

    #[test]
    fn insert_maintains_red_black_properties() {
        let mut tree = RedBlackTree::new();

        for i in 1..=15 {
            tree.insert(i);
            assert!(tree.is_valid_red_black_tree());
        }

        assert_eq!(tree.len(), 15);
    }

    #[test]
    fn sequential_insertion_stays_balanced() {
        let mut tree = RedBlackTree::new();

        for i in 1..=100 {
            tree.insert(i);
        }

        assert!(tree.is_valid_red_black_tree());
        assert!(tree.height() <= 2 * (tree.len() as f64).log2().ceil() as usize);
    }

    #[test]
    fn contains_operations() {
        let mut tree = RedBlackTree::new();
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
    fn iter_in_order() {
        let mut tree = RedBlackTree::new();
        for i in [4, 2, 6, 1, 3, 5, 7] {
            tree.insert(i);
        }

        let values: Vec<_> = tree.iter().cloned().collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn from_iterator() {
        let values = vec![5, 3, 7, 1, 9];
        let tree: RedBlackTree<_> = values.into_iter().collect();

        assert_eq!(tree.len(), 5);
        assert!(tree.is_valid_red_black_tree());
        for i in [1, 3, 5, 7, 9] {
            assert!(tree.contains(&i));
        }
    }

    #[test]
    fn stress_test() {
        let mut tree = RedBlackTree::new();

        for i in 0..1000 {
            tree.insert(i);
            assert!(tree.is_valid_red_black_tree());
        }

        assert_eq!(tree.len(), 1000);
        assert!(tree.height() <= 20);
    }
}
