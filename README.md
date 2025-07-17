# rand_set

[![Crates.io](https://img.shields.io/crates/v/rand_set.svg)](https://crates.io/crates/rand_set)
[![Documentation](https://docs.rs/rand_set/badge.svg)](https://docs.rs/rand_set)
[![License](https://img.shields.io/crates/l/rand_set.svg)](https://github.com/RonHachmon/rand_set#license)


A high-performance hash set implementation with **O(1) random element access** and all standard set operations.

## Features

- **O(1) Random Access**: Get random elements from the set in constant time
- **Standard Set Operations**: All the operations you expect from a hash set
- **Memory Efficient**: Optimized internal representation

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rand_set = "0.1"
```
## Usage

```rust
use rand_set::RandSet;

let mut set = RandSet::new();
set.insert("hello");
set.insert("world");

// Fast containment check (works like HashSet)
assert!(set.contains(&"hello"));

// Unique feature: Get a random element in O(1)
if let Some(random_item) = set.get_rand() {
    println!("Random: {}", random_item);
}
```

## License
This project is licensed under the MIT License - see the LICENSE file for details.