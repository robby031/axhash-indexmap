# axhash-indexmap

Fast insertion-ordered `IndexMap` / `IndexSet` powered by [`indexmap`](https://crates.io/crates/indexmap) and [`axhash`](https://crates.io/crates/axhash).

- Deterministic insertion-order iteration
- Fast hashing
- `IndexMap` + `IndexSet` compatible API
- `no_std + alloc` support
- Serde-friendly type aliases

[![Crates.io](https://img.shields.io/crates/v/axhash-indexmap.svg)](https://crates.io/crates/axhash-indexmap)
[![Docs.rs](https://docs.rs/axhash-indexmap/badge.svg)](https://docs.rs/axhash-indexmap)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Installation

```toml
[dependencies]
axhash-indexmap = "0.1"
```

### no_std + alloc

```toml
axhash-indexmap = { version = "0.1", default-features = false }
```

---

## Usage

### Type alias (`IndexMap` / `IndexSet`)

Best for Serde and drop-in compatibility.

```rust
use axhash_indexmap::IndexMap;

let mut map: IndexMap<&str, u32> = IndexMap::default();

map.insert("one", 1);
map.insert("two", 2);

assert_eq!(map.get_index(0), Some((&"one", &1)));
```

---

### Branded types (`AxIndexMap` / `AxIndexSet`)

Provides convenient constructors like `new()` and `with_capacity()`.

```rust
use axhash_indexmap::AxIndexMap;

let mut map: AxIndexMap<&str, u32> = AxIndexMap::new();

map.insert("one", 1);
map.insert("two", 2);

assert_eq!(map["one"], 1);
```

---

## Constructors

```rust
use axhash_indexmap::{
    AxIndexMap,
    AxIndexSet,
    new_map,
    new_set,
    map_with_capacity,
    set_with_capacity,
};

let map = new_map::<u32, u32>();
let set = new_set::<u32>();

let big_map = map_with_capacity::<u32, u32>(1024);
let big_set = set_with_capacity::<u32>(1024);

let map = AxIndexMap::<u32, u32>::new();
let set = AxIndexSet::<u32>::new();
```

---

## Custom seed

```rust
use axhash_indexmap::{AxBuildHasher, AxIndexMap};

let seed = 0xdeadbeef_cafebabe;

let map: AxIndexMap<u64, u64, AxBuildHasher> =
    AxIndexMap::with_hasher(
        AxBuildHasher::with_seed(seed)
    );
```

Use seeded hashing for untrusted input environments.

---

## Features

| Feature | Default | Description |
|---|:---:|---|
| `std` | ✅ | Standard library support |
| `no_std` | ❌ | Disable default features for `alloc`-only environments |

---

## Re-exports

```rust
use axhash_indexmap::{
    RawIndexMap,
    RawIndexSet,
    AxHasher,
    AxBuildHasher,
};
```

Useful when interoperating directly with raw `indexmap` types.

---

## Links

- Crate: https://crates.io/crates/axhash-indexmap
- Docs: https://docs.rs/axhash-indexmap
- indexmap: https://crates.io/crates/indexmap
- axhash: https://crates.io/crates/axhash

---

## License

MIT
