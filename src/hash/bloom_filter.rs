use crate::utils::{Clear, Size};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter<T> {
    bit_array: Vec<bool>,
    hash_count: usize,
    element_count: usize,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Hash> BloomFilter<T> {
    pub fn new(expected_elements: usize, false_positive_rate: f64) -> Self {
        let size = Self::optimal_size(expected_elements, false_positive_rate);
        let hash_count = Self::optimal_hash_count(size, expected_elements);

        Self {
            bit_array: vec![false; size],
            hash_count,
            element_count: 0,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn with_params(size: usize, hash_count: usize) -> Self {
        Self {
            bit_array: vec![false; size],
            hash_count,
            element_count: 0,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, item: &T) {
        for i in 0..self.hash_count {
            let hash = self.hash(item, i);
            let index = hash % self.bit_array.len();
            self.bit_array[index] = true;
        }
        self.element_count += 1;
    }

    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.hash_count {
            let hash = self.hash(item, i);
            let index = hash % self.bit_array.len();
            if !self.bit_array[index] {
                return false;
            }
        }
        true
    }

    pub fn false_positive_rate(&self) -> f64 {
        if self.element_count == 0 {
            return 0.0;
        }

        let k = self.hash_count as f64;
        let n = self.element_count as f64;
        let m = self.bit_array.len() as f64;

        (1.0 - (-k * n / m).exp()).powf(k)
    }

    pub fn bit_count(&self) -> usize {
        self.bit_array.iter().filter(|&&bit| bit).count()
    }

    pub fn capacity(&self) -> usize {
        self.bit_array.len()
    }

    pub fn hash_count(&self) -> usize {
        self.hash_count
    }

    fn hash(&self, item: &T, seed: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn optimal_size(expected_elements: usize, false_positive_rate: f64) -> usize {
        let n = expected_elements as f64;
        let p = false_positive_rate;

        let size = -(n * p.ln()) / (2.0_f64.ln().powi(2));
        size.ceil() as usize
    }

    fn optimal_hash_count(size: usize, expected_elements: usize) -> usize {
        if expected_elements == 0 {
            return 1;
        }

        let m = size as f64;
        let n = expected_elements as f64;

        let k = (m / n) * 2.0_f64.ln();
        k.round().max(1.0) as usize
    }
}

impl<T> Clear for BloomFilter<T> {
    fn clear(&mut self) {
        for bit in &mut self.bit_array {
            *bit = false;
        }
        self.element_count = 0;
    }
}

impl<T> Size for BloomFilter<T> {
    fn len(&self) -> usize {
        self.element_count
    }
}

impl<T: Hash> Default for BloomFilter<T> {
    fn default() -> Self {
        Self::new(1000, 0.01)
    }
}

impl<T: Hash> Extend<T> for BloomFilter<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.insert(&item);
        }
    }
}

impl<T: Hash> FromIterator<T> for BloomFilter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<_> = iter.into_iter().collect();
        let mut filter = BloomFilter::new(items.len(), 0.01);
        for item in items {
            filter.insert(&item);
        }
        filter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_filter_is_empty() {
        let filter: BloomFilter<i32> = BloomFilter::new(100, 0.01);
        assert!(filter.is_empty());
        assert_eq!(filter.len(), 0);
        assert_eq!(filter.bit_count(), 0);
        assert_eq!(filter.false_positive_rate(), 0.0);
    }

    #[test]
    fn insert_and_contains() {
        let mut filter = BloomFilter::new(100, 0.01);

        filter.insert(&42);
        filter.insert(&24);
        filter.insert(&36);

        assert!(filter.contains(&42));
        assert!(filter.contains(&24));
        assert!(filter.contains(&36));
        assert_eq!(filter.len(), 3);
    }

    #[test]
    fn definite_negatives() {
        let mut filter = BloomFilter::new(100, 0.01);

        filter.insert(&1);
        filter.insert(&2);
        filter.insert(&3);

        assert!(!filter.contains(&100));
        assert!(!filter.contains(&200));
        assert!(!filter.contains(&300));
    }

    #[test]
    fn false_positive_rate_calculation() {
        let mut filter = BloomFilter::new(10, 0.1);

        for i in 0..5 {
            filter.insert(&i);
        }

        let rate = filter.false_positive_rate();
        assert!(rate > 0.0);
        assert!(rate < 1.0);
    }

    #[test]
    fn optimal_parameters() {
        let filter = BloomFilter::<i32>::new(1000, 0.01);
        assert!(filter.capacity() > 0);
        assert!(filter.hash_count() > 0);

        let expected_size = BloomFilter::<i32>::optimal_size(1000, 0.01);
        let expected_hashes = BloomFilter::<i32>::optimal_hash_count(expected_size, 1000);

        assert_eq!(filter.capacity(), expected_size);
        assert_eq!(filter.hash_count(), expected_hashes);
    }

    #[test]
    fn bit_count_increases_with_insertions() {
        let mut filter = BloomFilter::new(100, 0.01);
        let initial_bits = filter.bit_count();

        filter.insert(&42);
        let after_one = filter.bit_count();

        filter.insert(&24);
        let after_two = filter.bit_count();

        assert!(after_one >= initial_bits);
        assert!(after_two >= after_one);
    }

    #[test]
    fn clear_resets_filter() {
        let mut filter = BloomFilter::new(100, 0.01);

        filter.insert(&1);
        filter.insert(&2);
        filter.insert(&3);

        assert!(!filter.is_empty());
        assert!(filter.bit_count() > 0);

        filter.clear();

        assert!(filter.is_empty());
        assert_eq!(filter.bit_count(), 0);
        assert!(!filter.contains(&1));
        assert!(!filter.contains(&2));
        assert!(!filter.contains(&3));
    }

    #[test]
    fn from_iterator() {
        let values = vec![1, 2, 3, 4, 5];
        let filter: BloomFilter<_> = values.iter().cloned().collect();

        assert_eq!(filter.len(), 5);
        for value in &values {
            assert!(filter.contains(value));
        }
    }

    #[test]
    fn extend_functionality() {
        let mut filter = BloomFilter::new(100, 0.01);
        let values = vec![10, 20, 30];

        filter.extend(values.iter().cloned());

        assert_eq!(filter.len(), 3);
        for value in &values {
            assert!(filter.contains(value));
        }
    }

    #[test]
    fn stress_test_no_false_negatives() {
        let mut filter = BloomFilter::new(1000, 0.01);
        let test_values: Vec<i32> = (0..500).collect();

        for value in &test_values {
            filter.insert(value);
        }

        for value in &test_values {
            assert!(
                filter.contains(value),
                "False negative for value: {}",
                value
            );
        }
    }

    #[test]
    fn false_positive_rate_within_bounds() {
        let mut filter = BloomFilter::new(100, 0.05);

        for i in 0..50 {
            filter.insert(&i);
        }

        let mut false_positives = 0;
        let test_range = 1000..2000;
        let test_count = test_range.len();

        for i in test_range {
            if filter.contains(&i) {
                false_positives += 1;
            }
        }

        let actual_rate = false_positives as f64 / test_count as f64;
        let theoretical_rate = filter.false_positive_rate();

        assert!(
            actual_rate <= theoretical_rate * 2.0,
            "Actual false positive rate {} exceeds theoretical bound {}",
            actual_rate,
            theoretical_rate
        );
    }
}
