//! HashSet implementation built on top of HashMap

use crate::hash::HashMap;
use crate::utils::{Clear, Size};
use std::fmt;
use std::hash::Hash;

/// A hash set implementation built on top of HashMap
///
/// # Examples
///
/// ```rust
/// use rust_ds_lib_bee::hash::HashSet;
/// use rust_ds_lib_bee::Size; // Import trait for len() method
///
/// let mut set = HashSet::new();
/// set.insert("value1");
/// set.insert("value2");
///
/// assert!(set.contains(&"value1"));
/// assert!(!set.contains(&"value3"));
/// assert_eq!(set.len(), 2);
/// ```
pub struct HashSet<T> {
    map: HashMap<T, ()>,
}

impl<T> HashSet<T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.map.remove(value).is_some()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            map_iter: self.map.keys(),
        }
    }

    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    pub fn load_factor(&self) -> f64 {
        self.map.load_factor()
    }

    pub fn union(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone,
    {
        let mut result = self.clone();
        for item in other.iter() {
            result.insert(item.clone());
        }
        result
    }

    pub fn intersection(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone,
    {
        let mut result = HashSet::new();
        for item in self.iter() {
            if other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    pub fn difference(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone,
    {
        let mut result = HashSet::new();
        for item in self.iter() {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    pub fn is_subset(&self, other: &HashSet<T>) -> bool {
        self.iter().all(|x| other.contains(x))
    }

    pub fn is_superset(&self, other: &HashSet<T>) -> bool {
        other.is_subset(self)
    }

    pub fn is_disjoint(&self, other: &HashSet<T>) -> bool {
        self.iter().all(|x| !other.contains(x))
    }
}

impl<T: Hash + Eq + Clone> Clone for HashSet<T> {
    fn clone(&self) -> Self {
        let mut result = HashSet::with_capacity(self.capacity());
        for item in self.iter() {
            result.insert(item.clone());
        }
        result
    }
}

impl<T: Hash + Eq> Default for HashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clear for HashSet<T> {
    fn clear(&mut self) {
        self.map.clear();
    }
}

impl<T> Size for HashSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T: fmt::Debug + Hash + Eq> fmt::Debug for HashSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

pub struct Iter<'a, T> {
    map_iter: crate::hash::hashmap::Keys<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next()
    }
}

impl<T: Hash + Eq> FromIterator<T> for HashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = HashSet::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

impl<T: Hash + Eq> Extend<T> for HashSet<T> {
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
    fn new_set_is_empty() {
        let set: HashSet<i32> = HashSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn insert_and_contains() {
        let mut set = HashSet::new();

        assert!(set.insert("value1"));
        assert!(!set.insert("value1"));
        assert!(set.insert("value2"));

        assert_eq!(set.len(), 2);
        assert!(set.contains(&"value1"));
        assert!(set.contains(&"value2"));
        assert!(!set.contains(&"value3"));
    }

    #[test]
    fn remove() {
        let mut set = HashSet::new();
        set.insert("value1");
        set.insert("value2");

        assert!(set.remove(&"value1"));
        assert!(!set.contains(&"value1"));
        assert_eq!(set.len(), 1);

        assert!(!set.remove(&"value3"));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn iter() {
        let mut set = HashSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");

        let mut values: Vec<_> = set.iter().cloned().collect();
        values.sort();
        assert_eq!(values, vec!["a", "b", "c"]);
    }

    #[test]
    fn set_operations() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        set1.insert(3);

        let mut set2 = HashSet::new();
        set2.insert(2);
        set2.insert(3);
        set2.insert(4);

        let subset: HashSet<_> = vec![1, 2].into_iter().collect();
        assert!(subset.is_subset(&set1));
        assert!(set1.is_superset(&subset));

        let disjoint: HashSet<_> = vec![5, 6].into_iter().collect();
        assert!(set1.is_disjoint(&disjoint));
        assert!(!set1.is_disjoint(&set2));
    }

    #[test]
    fn union_intersection_difference() {
        let set1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
        let set2: HashSet<_> = vec![2, 3, 4].into_iter().collect();

        let union = set1.union(&set2);
        assert_eq!(union.len(), 4);
        assert!(union.contains(&1));
        assert!(union.contains(&2));
        assert!(union.contains(&3));
        assert!(union.contains(&4));

        let intersection = set1.intersection(&set2);
        assert_eq!(intersection.len(), 2);
        assert!(intersection.contains(&2));
        assert!(intersection.contains(&3));

        let difference = set1.difference(&set2);
        assert_eq!(difference.len(), 1);
        assert!(difference.contains(&1));
    }

    #[test]
    fn from_iterator() {
        let values = vec![1, 2, 3, 2, 1];
        let set: HashSet<_> = values.into_iter().collect();

        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn clear() {
        let mut set = HashSet::new();
        set.insert("value1");
        set.insert("value2");

        assert!(!set.is_empty());
        set.clear();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }
}
