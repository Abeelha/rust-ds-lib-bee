use proptest::prelude::*;
use rust_ds_lib_bee::*;

prop_compose! {
    fn operations()(ops in prop::collection::vec(0..100i32, 0..1000)) -> Vec<i32> {
        ops
    }
}

proptest! {
    #[test]
    fn stack_operations_are_consistent(ops in operations()) {
        let mut stack = Stack::new();
        let mut reference = Vec::new();

        for op in ops {
            if op % 3 == 0 && !reference.is_empty() {
                let stack_result = stack.pop();
                let ref_result = reference.pop();
                prop_assert_eq!(stack_result, ref_result);
            } else {
                stack.push(op);
                reference.push(op);
            }
            
            prop_assert_eq!(stack.len(), reference.len());
            prop_assert_eq!(stack.is_empty(), reference.is_empty());
        }
    }

    #[test]
    fn bst_maintains_order_invariant(values in prop::collection::vec(0..1000i32, 0..100)) {
        let mut tree = BinarySearchTree::new();
        
        for value in values.iter() {
            tree.insert(*value);
        }
        
        let sorted_values: Vec<_> = tree.iter().cloned().collect();
        let mut expected = values.clone();
        expected.sort();
        expected.dedup();
        
        prop_assert_eq!(sorted_values, expected);
    }

    #[test]
    fn avl_tree_stays_balanced(values in prop::collection::vec(0..1000i32, 0..100)) {
        let mut tree = AvlTree::new();
        
        for value in values {
            tree.insert(value);
            prop_assert!(tree.is_balanced());
        }
    }

    #[test]
    fn hashmap_operations_consistent(
        keys in prop::collection::vec(0..1000i32, 0..100),
        values in prop::collection::vec(0..1000i32, 0..100)
    ) {
        let mut map = HashMap::new();
        let mut reference = std::collections::HashMap::new();
        
        for (k, v) in keys.iter().zip(values.iter()) {
            map.insert(*k, *v);
            reference.insert(*k, *v);
        }
        
        for k in keys.iter() {
            prop_assert_eq!(map.get(k), reference.get(k));
            prop_assert_eq!(map.contains_key(k), reference.contains_key(k));
        }
        
        prop_assert_eq!(map.len(), reference.len());
    }

    #[test]
    fn heap_maintains_heap_property(values in prop::collection::vec(0..1000i32, 0..100)) {
        let mut max_heap = BinaryHeap::max_heap();
        let mut min_heap = BinaryHeap::min_heap();
        
        for value in values.iter() {
            max_heap.push(*value);
            min_heap.push(*value);
        }
        
        let mut max_sorted = Vec::new();
        while let Some(val) = max_heap.pop() {
            max_sorted.push(val);
        }
        
        let mut min_sorted = Vec::new();
        while let Some(val) = min_heap.pop() {
            min_sorted.push(val);
        }
        
        for i in 1..max_sorted.len() {
            prop_assert!(max_sorted[i-1] >= max_sorted[i]);
        }
        
        for i in 1..min_sorted.len() {
            prop_assert!(min_sorted[i-1] <= min_sorted[i]);
        }
    }

    #[test]
    fn trie_prefix_properties(words in prop::collection::vec("[a-z]{1,10}", 0..50)) {
        let mut trie = Trie::new();
        
        for word in words.iter() {
            trie.insert(word);
        }
        
        for word in words.iter() {
            prop_assert!(trie.contains(word));
            
            for i in 1..=word.len() {
                let prefix = &word[..i];
                prop_assert!(trie.starts_with(prefix));
            }
        }
        
        let all_words = trie.all_words();
        prop_assert_eq!(all_words.len(), trie.len());
    }

    #[test]
    fn graph_connectivity_properties(edges in prop::collection::vec((0..20usize, 0..20usize), 0..50)) {
        let mut graph = Graph::directed();
        
        for (from, to) in edges.iter() {
            graph.add_edge(*from, *to);
        }
        
        for (from, to) in edges.iter() {
            prop_assert!(graph.has_edge(from, to));
            prop_assert!(graph.has_vertex(from));
            prop_assert!(graph.has_vertex(to));
        }
        
        let components = rust_ds_lib_bee::graph::algorithms::connected_components(&graph);
        let total_vertices: usize = components.iter().map(|c| c.len()).sum();
        prop_assert_eq!(total_vertices, graph.vertex_count());
    }

    #[test]
    fn bloom_filter_no_false_negatives(values in prop::collection::vec(0..1000i32, 0..100)) {
        let mut filter = BloomFilter::new(200, 0.01);
        
        for value in values.iter() {
            filter.insert(value);
        }
        
        for value in values.iter() {
            prop_assert!(filter.contains(value), "False negative for inserted value: {}", value);
        }
        
        prop_assert_eq!(filter.len(), values.len());
    }

    #[test]
    fn red_black_tree_maintains_properties(values in prop::collection::vec(0..1000i32, 0..100)) {
        let mut tree = RedBlackTree::new();
        
        for value in values.iter() {
            tree.insert(*value);
            prop_assert!(tree.is_valid_red_black_tree(), "Red-Black tree properties violated after inserting {}", value);
        }
        
        let sorted_values: Vec<_> = tree.iter().cloned().collect();
        let mut expected = values.clone();
        expected.sort();
        expected.dedup();
        
        prop_assert_eq!(sorted_values, expected);
    }
}