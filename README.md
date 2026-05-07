# axhash-indexmap

Insertion-ordered [`IndexMap`] and [`IndexSet`] collections powered by
[indexmap] (SwissTable + Vec layout) and fueled by [axhash] (AES-NI accelerated hashing).

[![Crates.io](https://img.shields.io/crates/v/axhash-indexmap.svg)](https://crates.io/crates/axhash-indexmap)
[![Docs.rs](https://docs.rs/axhash-indexmap/badge.svg)](https://docs.rs/axhash-indexmap)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Why axhash-indexmap?

`indexmap::IndexMap` defaults to **SipHash-1-3** — a secure but comparatively
slow hasher. Swapping in **axhash** (which exploits hardware AES instructions)
cuts hashing overhead dramatically while preserving everything that makes
`IndexMap` unique: deterministic insertion-order iteration and O(1) positional
access via `get_index`.

## Ecosystem

| Crate | Description |
|---|---|
| `axhash` | High-performance hashing engine |
| `axhash-map` | Fast HashMap/HashSet powered by `hashbrown` |
| `axhash-indexmap` | Ordered maps with AxHash |
| `axhash-dashmap` | Concurrent DashMap powered by AxHash |

```text
┌──────────────────────────────────────────────────────────┐
│                   axhash-indexmap                        │
│                                                          │
│  Type aliases (Serde-compatible)                         │
│  IndexMap<K, V>               IndexSet<T>                │
│                                                          │
│  Branded newtypes (ergonomic constructors)               │
│  AxIndexMap<K, V>             AxIndexSet<T>              │
│         │                           │                    │
│  indexmap::IndexMap      indexmap::IndexSet              │
│  (SwissTable lookup + Vec insertion-order slots)         │
│         │                           │                    │
│        BuildHasherDefault<AxHasher>                      │
│         (AES-NI accelerated hash engine)                 │
└──────────────────────────────────────────────────────────┘
```

---

## Two usage modes

This crate provides **two ways** to use the same fast hasher. Pick the one that fits your situation:

### Mode 1 — Type alias (`IndexMap` / `IndexSet`)

Plain type aliases over `indexmap` with `BuildHasherDefault<AxHasher>` baked in.
Because there is no wrapper struct, **Serde and other `#[derive]`-based crates work out of the box**.

```rust
use axhash_indexmap::IndexMap; // or IndexSet

// Works with serde::Serialize / Deserialize without any extra config.
let mut map: IndexMap<&str, u32> = IndexMap::default();
map.insert("insertion", 1);
map.insert("order",     2);
map.insert("preserved", 3);

// Insertion order guaranteed.
let keys: Vec<&str> = map.keys().copied().collect();
assert_eq!(keys, ["insertion", "order", "preserved"]);
```

### Mode 2 — Branded newtype (`AxIndexMap` / `AxIndexSet`)

A thin newtype wrapper that adds the familiar `::new()` / `::with_capacity()`
constructors (which `indexmap` 2.x only provides on the default-`RandomState`
form). Every `indexmap` method is accessible transparently via `Deref`.

```rust
use axhash_indexmap::AxIndexMap;

let mut map: AxIndexMap<&str, u32> = AxIndexMap::new();
map.insert("insertion", 1);
map.insert("order",     2);
map.insert("preserved", 3);

assert_eq!(map.get_index(0), Some((&"insertion", &1)));
```

| Need | Use |
|---|---|
| `::new()` / `::with_capacity()` | `AxIndexMap` / `AxIndexSet` |
| Serde `#[derive(Serialize, Deserialize)]` | `IndexMap` / `IndexSet` |
| Custom / seeded hasher | `AxIndexMap::with_hasher(AxBuildHasher::with_seed(s))` |
| Raw `indexmap` access | `RawIndexMap` / `RawIndexSet` |

---

## Benchmark Results

Measured on Apple Silicon (`release` build, `N = 100 000` items).

| Scenario | AxIndexMap | IndexMap (SipHash) | Speedup |
|---|---:|---:|:---:|
| Insert — `u64` keys | 998 µs | 1 905 µs | **1.9×** |
| Insert — `String` keys | 1 875 µs | 2 258 µs | **1.2×** |
| Lookup — all hits | 249 µs | 873 µs | **3.5×** |
| Lookup — 50 % hit / 50 % miss | 783 µs | 2 188 µs | **2.8×** |
| Iteration (full scan) | 19 µs | 20 µs | ~equal |
| Positional access (`get_index`) | 59 µs | 60 µs | ~equal |

> Iteration and `get_index` are draws — both are pure `Vec` operations that
> never call the hasher. The gains are concentrated entirely in the hashing
> step (insert + key lookup).

Run the benchmarks yourself:

```bash
cargo bench --bench indexmap_comparison
# HTML reports → target/criterion/
```

---

## Installation

```toml
[dependencies]
axhash-indexmap = "0.1"
```

No feature flags required. AES acceleration is detected at runtime; a portable
fallback is used automatically on CPUs without AES instructions.

For **no\_std + alloc** environments:

```toml
axhash-indexmap = { version = "0.1", default-features = false }
```

---

## Constructors

### Branded newtype constructors

> **⚠ indexmap quirk:** `indexmap` 2.x defines `IndexMap::new()` and
> `IndexMap::with_capacity()` **only** on `IndexMap<K, V>` (the default-hasher
> form), not on `IndexMap<K, V, S>` with a custom `S`. `AxIndexMap` exists
> precisely to solve this.

| Goal | Call |
|---|---|
| Empty map | `AxIndexMap::new()` or `new_map::<K, V>()` |
| Pre-allocated map | `AxIndexMap::with_capacity(n)` or `map_with_capacity::<K, V>(n)` |
| Empty set | `AxIndexMap::new()` or `new_set::<T>()` |
| Pre-allocated set | `AxIndexSet::with_capacity(n)` or `set_with_capacity::<T>(n)` |
| Default (zero seed) | `AxIndexMap::default()` |
| Custom seed | `AxIndexMap::with_hasher(AxBuildHasher::with_seed(s))` |
| Custom seed + capacity | `AxIndexMap::with_capacity_and_hasher(n, AxBuildHasher::with_seed(s))` |

### Type alias constructors

The `IndexMap` / `IndexSet` type aliases are plain `indexmap` types — use their
built-in `hashbrown`-backed constructors directly:

```rust
use axhash_indexmap::IndexMap;

let mut map: IndexMap<String, u32> = IndexMap::default();
let mut map = IndexMap::<String, u32>::with_capacity_and_hasher(
    1024,
    core::hash::BuildHasherDefault::default(),
);
```

---

## Quick start

### IndexMap (branded newtype)

```rust
use axhash_indexmap::{new_map, map_with_capacity, AxIndexMap};

// Empty map with axhash engine.
let mut map: AxIndexMap<&str, u32> = new_map();
map.insert("one",   1);
map.insert("two",   2);
map.insert("three", 3);

// Standard key lookup.
assert_eq!(map["one"], 1);
assert_eq!(map.get("two"), Some(&2));

// Iteration order == insertion order — guaranteed.
let keys: Vec<&str> = map.keys().copied().collect();
assert_eq!(keys, ["one", "two", "three"]);

// Positional access — unique to IndexMap.
assert_eq!(map.get_index(0), Some((&"one", &1)));
assert_eq!(map.get_index(2), Some((&"three", &3)));

// Pre-allocated map.
let mut big: AxIndexMap<u64, u64> = map_with_capacity(100_000);
for i in 0..100_000u64 {
    big.insert(i, i * i);
}
```

### IndexMap (type alias)

```rust
use axhash_indexmap::IndexMap;

// Construct and use exactly like indexmap::IndexMap — Serde-compatible.
let mut map: IndexMap<&str, u32> = IndexMap::default();
map.insert("one",   1);
map.insert("two",   2);
map.insert("three", 3);

// Insertion order + positional access still work — it's the same IndexMap.
assert_eq!(map.get_index(0), Some((&"one", &1)));
```

### IndexSet

```rust
use axhash_indexmap::{new_set, AxIndexSet};

let mut set: AxIndexSet<u32> = new_set();
set.insert(5);
set.insert(3);
set.insert(1);
set.insert(3); // duplicate — ignored, order preserved

// Insertion order preserved after dedup.
let order: Vec<u32> = set.iter().copied().collect();
assert_eq!(order, [5, 3, 1]);

// Positional access.
assert_eq!(set.get_index(0), Some(&5));

// Collect from iterator — duplicates dropped, first occurrence wins.
let set: AxIndexSet<u32> = [3, 1, 4, 1, 5].into_iter().collect();
assert_eq!(set.len(), 4);
```

---

## Seeded construction

Supply a random seed to harden against hash-flooding attacks when keys come
from untrusted input (e.g. HTTP parameters):

```rust
use axhash_indexmap::{AxIndexMap, AxBuildHasher};

// In production: derive seed from OS entropy (rand, getrandom, etc.).
let seed: u64 = 0xdeadbeef_cafebabe;

let mut map: AxIndexMap<String, usize, AxBuildHasher> =
    AxIndexMap::with_hasher(AxBuildHasher::with_seed(seed));
map.insert("hello".to_string(), 1);
assert_eq!(map["hello"], 1);

// With capacity + custom seed.
let mut map: AxIndexMap<String, usize, AxBuildHasher> =
    AxIndexMap::with_capacity_and_hasher(1024, AxBuildHasher::with_seed(seed));
```

---

## Entry API

```rust
use axhash_indexmap::new_map;

let mut word_count = new_map::<&str, u32>();

for word in ["apple", "banana", "apple", "cherry", "banana", "apple"] {
    word_count
        .entry(word)
        .and_modify(|n| *n += 1)
        .or_insert(1);
}

assert_eq!(word_count["apple"],  3);
assert_eq!(word_count["banana"], 2);
assert_eq!(word_count["cherry"], 1);

// Iteration order reflects first-insertion order of each key.
let order: Vec<&str> = word_count.keys().copied().collect();
assert_eq!(order, ["apple", "banana", "cherry"]);
```

---

## Collecting and extending

```rust
use axhash_indexmap::{AxIndexMap, AxIndexSet};

// FromIterator for maps.
let map: AxIndexMap<&str, u32> = [("a", 1), ("b", 2), ("c", 3)]
    .into_iter()
    .collect();

// FromIterator for sets — duplicates dropped.
let set: AxIndexSet<i32> = [1, 2, 3, 2, 1].into_iter().collect();
assert_eq!(set.len(), 3);

// Extend.
let mut map: AxIndexMap<u32, u32> = AxIndexMap::default();
map.extend([(1, 10), (2, 20)]);
map.extend([(3, 30)]);
assert_eq!(map.len(), 3);
```

---

## Sorting

```rust
use axhash_indexmap::new_map;

let mut map = new_map::<&str, u32>();
map.insert("banana", 2);
map.insert("apple",  1);
map.insert("cherry", 3);

// Sort in-place by key.
map.sort_keys();
let keys: Vec<&str> = map.keys().copied().collect();
assert_eq!(keys, ["apple", "banana", "cherry"]);

// Sort by value.
map.sort_by(|_, a, _, b| a.cmp(b));
```

---

## Set operations

```rust
use axhash_indexmap::{new_set, AxIndexSet};

let a: AxIndexSet<u32> = [1, 2, 3, 4].into_iter().collect();
let b: AxIndexSet<u32> = [3, 4, 5, 6].into_iter().collect();

let union:     AxIndexSet<u32> = a.union(&b).copied().collect();
let inter:     AxIndexSet<u32> = a.intersection(&b).copied().collect();
let diff:      AxIndexSet<u32> = a.difference(&b).copied().collect();
let sym_diff:  AxIndexSet<u32> = a.symmetric_difference(&b).copied().collect();

assert_eq!(union.len(),    6);
assert_eq!(inter.len(),    2);
assert_eq!(diff.len(),     2);
assert_eq!(sym_diff.len(), 4);
```

---

## Interoperability with raw indexmap types

The crate re-exports `RawIndexMap` and `RawIndexSet` so you can interoperate
with the underlying indexmap types without a direct `indexmap` dependency.
`AxIndexMap<K, V>` converts to/from `RawIndexMap<K, V, S>` via `From`:

```rust
use core::hash::BuildHasherDefault;
use axhash_indexmap::{AxIndexMap, RawIndexMap, AxHasher};

// Build a raw indexmap with BuildHasherDefault<AxHasher>.
let mut raw: RawIndexMap<&str, i32, BuildHasherDefault<AxHasher>> =
    RawIndexMap::with_hasher(BuildHasherDefault::default());
raw.insert("x", 99);

// Convert into the branded newtype.
let ax: AxIndexMap<&str, i32> = raw.into();
assert_eq!(ax["x"], 99);

// And back.
let raw: RawIndexMap<&str, i32, BuildHasherDefault<AxHasher>> = ax.into();
assert_eq!(raw["x"], 99);
```

---

## no\_std support

Disable the `std` feature to target `no_std + alloc` environments:

```toml
axhash-indexmap = { version = "0.1", default-features = false }
```

Pure `no_std` without heap allocation is not supported because `IndexMap`
requires dynamic memory.

---

## Feature flags

| Flag | Default | Effect |
|---|:---:|---|
| `std` | ✅ on | Links against std. Disable for no\_std + alloc. |

---

## Dependency footprint

```text
axhash-indexmap
├── axhash-core   (AxHasher + AxBuildHasher — AES hash engine, no std dependency)
└── indexmap      (SwissTable + Vec, no default features)
```

`axhash-map` is **not** a dependency — `AxHasher` lives in `axhash-core`
directly. Adding `axhash-map` would pull in hashbrown for no benefit.

---

## License

MIT — see [LICENSE](LICENSE).

[`IndexMap`]: https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html
[`IndexSet`]: https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html
[indexmap]: https://crates.io/crates/indexmap
[axhash]: https://crates.io/crates/axhash
