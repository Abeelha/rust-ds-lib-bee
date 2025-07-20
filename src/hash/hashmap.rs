use crate::utils::{Clear, Size};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

const DEFAULT_CAPACITY: usize = 16;

const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

#[derive(Debug, Clone)]
struct Entry<K, V> {
    key: K,
    value: V,
    next: Option<Box<Entry<K, V>>>,
}

impl<K, V> Entry<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            next: None,
        }
    }
}

pub struct HashMap<K, V> {
    buckets: Vec<Option<Box<Entry<K, V>>>>,
    size: usize,
    capacity: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Self {
            buckets: (0..capacity).map(|_| None).collect(),
            size: 0,
            capacity,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.should_resize() {
            self.resize();
        }

        let index = self.hash(&key);

        {
            let mut current = &mut self.buckets[index];
            while let Some(ref mut entry) = current {
                if entry.key == key {
                    let old_value = std::mem::replace(&mut entry.value, value);
                    return Some(old_value);
                }
                current = &mut entry.next;
            }
        }

        let bucket = &mut self.buckets[index];
        let mut new_entry = Box::new(Entry::new(key, value));
        new_entry.next = bucket.take();
        *bucket = Some(new_entry);
        self.size += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        let mut current = &self.buckets[index];

        while let Some(ref entry) = current {
            if entry.key == *key {
                return Some(&entry.value);
            }
            current = &entry.next;
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.hash(key);
        let mut current = &mut self.buckets[index];

        while let Some(ref mut entry) = current {
            if entry.key == *key {
                return Some(&mut entry.value);
            }
            current = &mut entry.next;
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];

        if let Some(ref entry) = bucket {
            if entry.key == *key {
                let removed = bucket.take().unwrap();
                *bucket = removed.next;
                self.size -= 1;
                return Some(removed.value);
            }
        }

        let mut current = bucket;
        while let Some(ref mut entry) = current {
            if let Some(ref next_entry) = entry.next {
                if next_entry.key == *key {
                    let removed = entry.next.take().unwrap();
                    entry.next = removed.next;
                    self.size -= 1;
                    return Some(removed.value);
                }
            }
            current = &mut entry.next;
        }
        None
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            bucket_iter: self.buckets.iter(),
            current_chain: None,
        }
    }

    pub fn keys(&self) -> Keys<K, V> {
        Keys { iter: self.iter() }
    }

    pub fn values(&self) -> Values<K, V> {
        Values { iter: self.iter() }
    }

    pub fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    fn should_resize(&self) -> bool {
        self.load_factor() > LOAD_FACTOR_THRESHOLD
    }

    fn resize(&mut self) {
        let old_buckets = std::mem::replace(
            &mut self.buckets,
            (0..self.capacity * 2).map(|_| None).collect(),
        );
        let _old_capacity = self.capacity;
        self.capacity *= 2;
        self.size = 0;

        for bucket in old_buckets {
            let mut current = bucket;
            while let Some(entry) = current {
                let Entry { key, value, next } = *entry;
                self.insert(key, value);
                current = next;
            }
        }
    }
}

impl<K: Hash + Eq, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clear for HashMap<K, V> {
    fn clear(&mut self) {
        for bucket in &mut self.buckets {
            *bucket = None;
        }
        self.size = 0;
    }
}

impl<K, V> Size for HashMap<K, V> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<K: fmt::Debug + Hash + Eq, V: fmt::Debug> fmt::Debug for HashMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

pub struct Iter<'a, K, V> {
    bucket_iter: std::slice::Iter<'a, Option<Box<Entry<K, V>>>>,
    current_chain: Option<&'a Entry<K, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entry) = self.current_chain {
                let result = (&entry.key, &entry.value);
                self.current_chain = entry.next.as_deref();
                return Some(result);
            }

            match self.bucket_iter.next() {
                Some(Some(entry)) => {
                    self.current_chain = Some(entry);
                }
                Some(None) => continue,
                None => return None,
            }
        }
    }
}

pub struct Keys<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

pub struct Values<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for HashMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = HashMap::new();
        for (key, value) in iter {
            map.insert(key, value);
        }
        map
    }
}

impl<K: Hash + Eq, V> Extend<(K, V)> for HashMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_map_is_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), DEFAULT_CAPACITY);
    }

    #[test]
    fn insert_and_get() {
        let mut map = HashMap::new();

        assert_eq!(map.insert("key1", "value1"), None);
        assert_eq!(map.insert("key2", "value2"), None);
        assert_eq!(map.insert("key1", "new_value"), Some("value1"));

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&"key1"), Some(&"new_value"));
        assert_eq!(map.get(&"key2"), Some(&"value2"));
        assert_eq!(map.get(&"key3"), None);
    }

    #[test]
    fn contains_key() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        assert!(map.contains_key(&"key1"));
        assert!(map.contains_key(&"key2"));
        assert!(!map.contains_key(&"key3"));
    }

    #[test]
    fn remove() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        map.insert("key3", "value3");

        assert_eq!(map.remove(&"key2"), Some("value2"));
        assert_eq!(map.len(), 2);
        assert!(!map.contains_key(&"key2"));

        assert_eq!(map.remove(&"key4"), None);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn get_mut() {
        let mut map = HashMap::new();
        map.insert("key1", 10);

        if let Some(value) = map.get_mut(&"key1") {
            *value += 5;
        }

        assert_eq!(map.get(&"key1"), Some(&15));
    }

    #[test]
    fn iter() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        map.insert("key3", "value3");

        let mut pairs: Vec<_> = map.iter().collect();
        pairs.sort_by_key(|(k, _)| *k);

        assert_eq!(
            pairs,
            vec![
                (&"key1", &"value1"),
                (&"key2", &"value2"),
                (&"key3", &"value3"),
            ]
        );
    }

    #[test]
    fn keys_and_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let mut keys: Vec<_> = map.keys().cloned().collect();
        keys.sort();
        assert_eq!(keys, vec!["a", "b", "c"]);

        let mut values: Vec<_> = map.values().cloned().collect();
        values.sort();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn resize_on_load_factor() {
        let mut map = HashMap::with_capacity(4);

        for i in 0..10 {
            map.insert(i, i * 10);
        }

        assert!(map.capacity() > 4);
        assert_eq!(map.len(), 10);

        for i in 0..10 {
            assert_eq!(map.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    fn from_iterator() {
        let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
        let map: HashMap<_, _> = pairs.into_iter().collect();

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"c"), Some(&3));
    }

    #[test]
    fn clear() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        assert!(!map.is_empty());
        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert!(!map.contains_key(&"key1"));
    }

    #[test]
    fn collision_handling() {
        let mut map = HashMap::with_capacity(2);

        for i in 0..20 {
            map.insert(i, i * 100);
        }

        for i in 0..20 {
            assert_eq!(map.get(&i), Some(&(i * 100)));
        }
        assert_eq!(map.len(), 20);
    }
}
