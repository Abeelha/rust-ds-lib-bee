use crate::utils::{Clear, Size};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
    word_count: usize,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
            word_count: 0,
        }
    }

    pub fn insert(&mut self, word: &str) -> bool {
        let mut current = &mut self.root;
        
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }
        
        if current.is_end_of_word {
            false
        } else {
            current.is_end_of_word = true;
            self.word_count += 1;
            true
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        self.find_node(word).is_some_and(|node| node.is_end_of_word)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    pub fn remove(&mut self, word: &str) -> bool {
        if self.contains(word) {
            Self::remove_recursive_static(&mut self.root, word, 0);
            self.word_count -= 1;
            true
        } else {
            false
        }
    }

    fn remove_recursive_static(node: &mut TrieNode, word: &str, index: usize) -> bool {
        if index == word.len() {
            if node.is_end_of_word {
                node.is_end_of_word = false;
                return node.children.is_empty();
            }
            return false;
        }

        let ch = word.chars().nth(index).unwrap();
        
        if let Some(child) = node.children.get_mut(&ch) {
            let should_delete_child = Self::remove_recursive_static(child, word, index + 1);
            
            if should_delete_child {
                node.children.remove(&ch);
            }
            
            return !node.is_end_of_word && node.children.is_empty();
        }
        
        false
    }

    pub fn find_words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        
        if let Some(prefix_node) = self.find_node(prefix) {
            Self::collect_words(prefix_node, prefix, &mut result);
        }
        
        result
    }

    fn collect_words(node: &TrieNode, current_word: &str, result: &mut Vec<String>) {
        if node.is_end_of_word {
            result.push(current_word.to_string());
        }

        for (ch, child_node) in &node.children {
            let mut next_word = current_word.to_string();
            next_word.push(*ch);
            Self::collect_words(child_node, &next_word, result);
        }
    }

    fn find_node(&self, word: &str) -> Option<&TrieNode> {
        let mut current = &self.root;
        
        for ch in word.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return None,
            }
        }
        
        Some(current)
    }

    pub fn word_count(&self) -> usize {
        self.word_count
    }

    pub fn all_words(&self) -> Vec<String> {
        let mut result = Vec::new();
        Self::collect_words(&self.root, "", &mut result);
        result
    }

    pub fn longest_common_prefix(&self) -> String {
        let mut result = String::new();
        let mut current = &self.root;

        while current.children.len() == 1 && !current.is_end_of_word {
            let (ch, child) = current.children.iter().next().unwrap();
            result.push(*ch);
            current = child;
        }

        result
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Clear for Trie {
    fn clear(&mut self) {
        self.root = TrieNode::new();
        self.word_count = 0;
    }
}

impl Size for Trie {
    fn len(&self) -> usize {
        self.word_count
    }
}

impl fmt::Debug for Trie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Trie")
            .field("word_count", &self.word_count)
            .field("words", &self.all_words())
            .finish()
    }
}

impl FromIterator<String> for Trie {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut trie = Trie::new();
        for word in iter {
            trie.insert(&word);
        }
        trie
    }
}

impl<'a> FromIterator<&'a str> for Trie {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut trie = Trie::new();
        for word in iter {
            trie.insert(word);
        }
        trie
    }
}

impl Extend<String> for Trie {
    fn extend<I: IntoIterator<Item = String>>(&mut self, iter: I) {
        for word in iter {
            self.insert(&word);
        }
    }
}

impl<'a> Extend<&'a str> for Trie {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
        for word in iter {
            self.insert(word);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_trie_is_empty() {
        let trie = Trie::new();
        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);
        assert_eq!(trie.word_count(), 0);
    }

    #[test]
    fn insert_and_contains() {
        let mut trie = Trie::new();
        
        assert!(trie.insert("hello"));
        assert!(!trie.insert("hello"));
        assert!(trie.insert("world"));
        assert!(trie.insert("help"));

        assert_eq!(trie.len(), 3);
        assert!(trie.contains("hello"));
        assert!(trie.contains("world"));
        assert!(trie.contains("help"));
        assert!(!trie.contains("he"));
        assert!(!trie.contains("helloworld"));
    }

    #[test]
    fn starts_with() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("world");

        assert!(trie.starts_with("hel"));
        assert!(trie.starts_with("hell"));
        assert!(trie.starts_with("help"));
        assert!(trie.starts_with("wor"));
        assert!(!trie.starts_with("abc"));
    }

    #[test]
    fn remove_words() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("world");

        assert!(trie.remove("hello"));
        assert!(!trie.contains("hello"));
        assert!(trie.contains("help"));
        assert_eq!(trie.len(), 2);

        assert!(!trie.remove("nonexistent"));
        assert_eq!(trie.len(), 2);
    }

    #[test]
    fn find_words_with_prefix() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("helper");
        trie.insert("world");

        let words = trie.find_words_with_prefix("hel");
        assert_eq!(words.len(), 3);
        assert!(words.contains(&"hello".to_string()));
        assert!(words.contains(&"help".to_string()));
        assert!(words.contains(&"helper".to_string()));

        let words = trie.find_words_with_prefix("wor");
        assert_eq!(words.len(), 1);
        assert!(words.contains(&"world".to_string()));

        let words = trie.find_words_with_prefix("xyz");
        assert!(words.is_empty());
    }

    #[test]
    fn all_words() {
        let mut trie = Trie::new();
        trie.insert("cat");
        trie.insert("car");
        trie.insert("card");

        let words = trie.all_words();
        assert_eq!(words.len(), 3);
        assert!(words.contains(&"cat".to_string()));
        assert!(words.contains(&"car".to_string()));
        assert!(words.contains(&"card".to_string()));
    }

    #[test]
    fn longest_common_prefix() {
        let mut trie = Trie::new();
        trie.insert("flower");
        trie.insert("flow");
        trie.insert("flight");

        assert_eq!(trie.longest_common_prefix(), "fl");

        let mut single_word = Trie::new();
        single_word.insert("hello");
        assert_eq!(single_word.longest_common_prefix(), "hello");
    }

    #[test]
    fn from_iterator() {
        let words = vec!["hello", "world", "help"];
        let trie: Trie = words.into_iter().collect();
        
        assert_eq!(trie.len(), 3);
        assert!(trie.contains("hello"));
        assert!(trie.contains("world"));
        assert!(trie.contains("help"));
    }

    #[test]
    fn clear_trie() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");

        assert!(!trie.is_empty());
        trie.clear();
        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);
        assert!(!trie.contains("hello"));
    }

    #[test]
    fn edge_cases() {
        let mut trie = Trie::new();
        
        assert!(trie.insert(""));
        assert!(trie.contains(""));
        assert_eq!(trie.len(), 1);

        assert!(trie.insert("a"));
        assert!(trie.starts_with(""));
        assert!(trie.starts_with("a"));
    }
}