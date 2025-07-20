# Contributing to rust-ds-lib-bee

We're excited that you're interested in contributing to `rust-ds-lib-bee`! This library serves both as a practical tool and an educational resource for learning data structures and algorithms in Rust.

## Ways to Contribute

- **Bug Reports**: Found an issue? Let me know!
- **Feature Requests**: Ideas for new data structures or algorithms
- **Code Contributions**: Implement new structures, fix bugs, or improve performance
- **Documentation**: Improve examples, explanations, or educational content
- **Testing**: Add test cases, property-based tests, or benchmarks
- **Educational Content**: Better explanations of algorithms and complexity analysis

## Getting Started

### Development Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/abeelha/rust-ds-lib-bee.git
   cd rust-ds-lib-bee
   ```

2. **Install Rust** (if not already installed)

   ```bash
   # Install via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **Verify your setup**
   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features
   cargo fmt --all -- --check
   cargo bench
   ```

### Project Structure

```
src/
├── linear/          # Stack, Queue, LinkedList
├── tree/           # BST, AVL, RedBlack, Trie
├── hash/           # HashMap, HashSet, BloomFilter
├── heap/           # BinaryHeap, PriorityQueue
├── graph/          # Graph, WeightedGraph, algorithms
└── utils/          # Shared traits and utilities

benches/            # Performance benchmarks
tests/              # Integration and property tests
examples/           # Usage examples (planned)
```

## Coding Standards

### Code Quality Requirements

1. **Safety First**

   - No `unsafe` code unless absolutely necessary and well-documented
   - Prefer `Option` and `Result` over panics
   - Handle all error cases gracefully

2. **Performance**

   - Implement optimal time complexities
   - Include complexity analysis in documentation
   - Add benchmarks for new data structures

3. **Documentation**

   - Every public function must have doc comments
   - Include examples in doc comments
   - Document time and space complexity
   - Explain algorithm choices and trade-offs

4. **Testing**
   - Unit tests for all public functions
   - Property-based tests for mathematical invariants
   - Edge case coverage
   - Benchmarks for performance regression detection

### Code Style

We follow standard Rust conventions:

```bash
# Format code
cargo fmt

# Check for common issues
cargo clippy --all-targets --all-features -- -D warnings

# Check documentation
cargo doc --all-features --no-deps
```

### Commit Message Format

Use clear, descriptive commit messages:

```
Add Red-Black Tree implementation with balancing

- Implement insert with proper rotations
- Add color-based balancing logic
- Include comprehensive property tests
- Document O(log n) guarantees
```

## Adding New Data Structures

When contributing a new data structure:

### 1. Implementation Requirements

- **Generic**: Support any type that satisfies necessary bounds
- **Traits**: Implement relevant traits from `utils::traits`
- **Iterator**: Provide iterator support where applicable
- **Documentation**: Include complexity analysis and examples

### 2. File Organization

````rust
// src/category/new_structure.rs
use crate::utils::{Clear, Size, /* other traits */};

/// Brief description of the data structure
///
/// # Time Complexity
/// - Operation1: O(complexity)
/// - Operation2: O(complexity)
///
/// # Space Complexity: O(complexity)
///
/// # Examples
/// ```rust
/// // Usage example
/// ```
pub struct NewStructure<T> {
    // Implementation
}

impl<T> NewStructure<T> {
    /// Creates a new instance
    pub fn new() -> Self {
        // Implementation
    }

    // Other methods...
}

// Trait implementations
impl<T> Clear for NewStructure<T> { /* */ }
impl<T> Size for NewStructure<T> { /* */ }

#[cfg(test)]
mod tests {
    // Comprehensive tests
}
````

### 3. Module Integration

Update the appropriate `mod.rs` file:

```rust
// src/category/mod.rs
pub mod new_structure;
pub use new_structure::NewStructure;
```

Update `src/lib.rs`:

```rust
pub use category::NewStructure;
```

### 4. Testing Requirements

- **Unit Tests**: Test all public methods
- **Property Tests**: Add to `tests/property_tests.rs`
- **Benchmarks**: Create performance benchmarks
- **Integration Tests**: Test interaction with other structures

### 5. Documentation Updates

- Update README.md feature list
- Add usage examples
- Update complexity analysis tables

## Testing Guidelines

### Unit Tests

```rust
#[test]
fn test_basic_operations() {
    let mut structure = NewStructure::new();
    assert!(structure.is_empty());

    structure.insert(42);
    assert!(!structure.is_empty());
    assert_eq!(structure.len(), 1);
}
```

### Property-Based Tests

```rust
// tests/property_tests.rs
proptest! {
    #[test]
    fn structure_maintains_invariant(values in prop::collection::vec(0..1000i32, 0..100)) {
        let mut structure = NewStructure::new();

        for value in values.iter() {
            structure.insert(*value);
            // Assert invariants
        }
    }
}
```

### Benchmarks

```rust
// benches/new_structure_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_insert(c: &mut Criterion) {
    c.bench_function("new_structure_insert_1000", |b| {
        b.iter(|| {
            let mut structure = NewStructure::new();
            for i in 0..1000 {
                structure.insert(black_box(i));
            }
        })
    });
}

criterion_group!(benches, bench_insert);
criterion_main!(benches);
```

## Pull Request Process

### Before Submitting

1. **Run all checks**

   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   cargo bench
   ```

2. **Update documentation**

   - Add/update doc comments
   - Update README if needed
   - Include examples

3. **Add tests**
   - Unit tests for new functionality
   - Property tests for invariants
   - Benchmarks for performance

### PR Requirements

- **Clear Description**: Explain what you're adding/fixing and why
- **Issue Reference**: Link to related issues if applicable
- **Testing**: All tests must pass
- **Documentation**: New features must be documented
- **No Breaking Changes**: Unless discussed in an issue first

### Review Process

1. Automated checks must pass (CI/CD)
2. Code review focusing on:
   - Correctness and safety
   - Performance implications
   - API design consistency
   - Documentation quality
3. Educational value assessment
4. Approval and merge

## Issue Reporting

### Bug Reports

Include:

- Rust version and platform
- Minimal reproduction case
- Expected vs actual behavior
- Stack trace if applicable

### Feature Requests

Include:

- Description of the data structure/algorithm
- Use cases and benefits
- Time/space complexity expectations
- Reference materials (papers, books, etc.)

## Educational Focus

Remember that this library prioritizes:

1. **Learning**: Clear, understandable implementations
2. **Correctness**: Mathematically sound algorithms
3. **Safety**: Memory-safe Rust practices
4. **Performance**: Optimal complexity without sacrificing clarity
5. **Documentation**: Thorough explanations and examples

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and understand
- Celebrate good implementations
- Ask questions when unsure

## Recognition

Contributors will be:

- Listed in [AUTHORS](AUTHORS.md)
- Mentioned in release notes for significant contributions
- Credited in documentation for major features

## Getting Help

- **Discussions**: [GitHub Discussions](https://github.com/abeelha/rust-ds-lib-bee/discussions) for questions
- **Issues**: [Create an Issue](https://github.com/abeelha/rust-ds-lib-bee/issues/new) for bugs or feature requests

## Resources

### Learning Materials

- [Rust Documentation](https://doc.rust-lang.org/stable/std/index.html)
- [The Algorithm Design Manual](https://www.algorist.com/)
- [Introduction to Algorithms (CLRS)](https://mitpress.mit.edu/books/introduction-algorithms-third-edition)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Rust-Specific

- [API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Property-Based Testing](https://proptest-rs.github.io/proptest/)

Thank you for contributing to `rust-ds-lib-bee`! Together, we're building both a practical tool and an educational resource for the Rust community.
