#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use core::fmt;
use core::hash::{BuildHasher, BuildHasherDefault, Hash};
use core::ops::{Deref, DerefMut, Index};

pub use axhash_core::{AxBuildHasher, AxHasher};
pub use indexmap::IndexMap as RawIndexMap;
pub use indexmap::IndexSet as RawIndexSet;
pub use indexmap::map::Entry as MapEntry;
pub use indexmap::map::OccupiedEntry as MapOccupiedEntry;
pub use indexmap::map::VacantEntry as MapVacantEntry;

pub type IndexMap<K, V> = RawIndexMap<K, V, BuildHasherDefault<AxHasher>>;
pub type IndexSet<T> = RawIndexSet<T, BuildHasherDefault<AxHasher>>;

pub struct AxIndexMap<K, V, S = BuildHasherDefault<AxHasher>>(RawIndexMap<K, V, S>);

impl<K, V> AxIndexMap<K, V, BuildHasherDefault<AxHasher>> {
    #[inline(always)]
    pub fn new() -> Self {
        Self(RawIndexMap::with_hasher(BuildHasherDefault::default()))
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(RawIndexMap::with_capacity_and_hasher(
            capacity,
            BuildHasherDefault::default(),
        ))
    }
}

impl<K, V, S: BuildHasher> AxIndexMap<K, V, S> {
    #[inline(always)]
    pub fn with_hasher(hasher: S) -> Self {
        Self(RawIndexMap::with_hasher(hasher))
    }

    #[inline(always)]
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self(RawIndexMap::with_capacity_and_hasher(capacity, hasher))
    }

    #[inline(always)]
    pub fn into_inner(self) -> RawIndexMap<K, V, S> {
        self.0
    }
}

impl<K, V, S> Deref for AxIndexMap<K, V, S> {
    type Target = RawIndexMap<K, V, S>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V, S> DerefMut for AxIndexMap<K, V, S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> Default for AxIndexMap<K, V, BuildHasherDefault<AxHasher>> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<K: fmt::Debug, V: fmt::Debug, S> fmt::Debug for AxIndexMap<K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<K: Clone, V: Clone, S: Clone> Clone for AxIndexMap<K, V, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K: Hash + Eq, V: PartialEq, S: BuildHasher> PartialEq for AxIndexMap<K, V, S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K: Hash + Eq, V: Eq, S: BuildHasher> Eq for AxIndexMap<K, V, S> {}

impl<K, Q, V, S> Index<&Q> for AxIndexMap<K, V, S>
where
    K: Hash + Eq + core::borrow::Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    S: BuildHasher,
{
    type Output = V;

    #[inline]
    fn index(&self, key: &Q) -> &Self::Output {
        self.0.index(key)
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for AxIndexMap<K, V, BuildHasherDefault<AxHasher>> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        let mut map = Self::with_capacity(lower);
        map.extend(iter);
        map
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> Extend<(K, V)> for AxIndexMap<K, V, S> {
    #[inline]
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<K, V, S> IntoIterator for AxIndexMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = indexmap::map::IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a AxIndexMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = indexmap::map::Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut AxIndexMap<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = indexmap::map::IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<K, V, S> From<RawIndexMap<K, V, S>> for AxIndexMap<K, V, S> {
    #[inline]
    fn from(inner: RawIndexMap<K, V, S>) -> Self {
        Self(inner)
    }
}

impl<K, V, S> From<AxIndexMap<K, V, S>> for RawIndexMap<K, V, S> {
    #[inline]
    fn from(wrapper: AxIndexMap<K, V, S>) -> Self {
        wrapper.0
    }
}

pub struct AxIndexSet<T, S = BuildHasherDefault<AxHasher>>(RawIndexSet<T, S>);

impl<T> AxIndexSet<T, BuildHasherDefault<AxHasher>> {
    #[inline(always)]
    pub fn new() -> Self {
        Self(RawIndexSet::with_hasher(BuildHasherDefault::default()))
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(RawIndexSet::with_capacity_and_hasher(
            capacity,
            BuildHasherDefault::default(),
        ))
    }
}

impl<T, S: BuildHasher> AxIndexSet<T, S> {
    #[inline(always)]
    pub fn with_hasher(hasher: S) -> Self {
        Self(RawIndexSet::with_hasher(hasher))
    }

    #[inline(always)]
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self(RawIndexSet::with_capacity_and_hasher(capacity, hasher))
    }

    #[inline(always)]
    pub fn into_inner(self) -> RawIndexSet<T, S> {
        self.0
    }
}

impl<T, S> Deref for AxIndexSet<T, S> {
    type Target = RawIndexSet<T, S>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> DerefMut for AxIndexSet<T, S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for AxIndexSet<T, BuildHasherDefault<AxHasher>> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug, S> fmt::Debug for AxIndexSet<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Clone, S: Clone> Clone for AxIndexSet<T, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Hash + Eq, S: BuildHasher> PartialEq for AxIndexSet<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Hash + Eq, S: BuildHasher> Eq for AxIndexSet<T, S> {}

impl<T: Hash + Eq> FromIterator<T> for AxIndexSet<T, BuildHasherDefault<AxHasher>> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        let mut set = Self::with_capacity(lower);
        set.extend(iter);
        set
    }
}

impl<T: Hash + Eq, S: BuildHasher> Extend<T> for AxIndexSet<T, S> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<T, S> IntoIterator for AxIndexSet<T, S> {
    type Item = T;
    type IntoIter = indexmap::set::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, S> IntoIterator for &'a AxIndexSet<T, S> {
    type Item = &'a T;
    type IntoIter = indexmap::set::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T, S> From<RawIndexSet<T, S>> for AxIndexSet<T, S> {
    #[inline]
    fn from(inner: RawIndexSet<T, S>) -> Self {
        Self(inner)
    }
}

impl<T, S> From<AxIndexSet<T, S>> for RawIndexSet<T, S> {
    #[inline]
    fn from(wrapper: AxIndexSet<T, S>) -> Self {
        wrapper.0
    }
}

#[inline(always)]
pub fn new_map<K, V>() -> AxIndexMap<K, V> {
    AxIndexMap::new()
}

#[inline(always)]
pub fn map_with_capacity<K, V>(capacity: usize) -> AxIndexMap<K, V> {
    AxIndexMap::with_capacity(capacity)
}

#[inline(always)]
pub fn new_set<T>() -> AxIndexSet<T> {
    AxIndexSet::new()
}

#[inline(always)]
pub fn set_with_capacity<T>(capacity: usize) -> AxIndexSet<T> {
    AxIndexSet::with_capacity(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::hash::BuildHasherDefault;

    #[test]
    fn map_constructors() {
        let m1: AxIndexMap<&str, u32> = AxIndexMap::default();
        let m2: AxIndexMap<&str, u32> = AxIndexMap::new();
        let m3: AxIndexMap<&str, u32> = new_map();
        let m4: AxIndexMap<&str, u32> = AxIndexMap::with_hasher(BuildHasherDefault::default());
        assert!(m1.is_empty() && m2.is_empty() && m3.is_empty() && m4.is_empty());
    }

    #[test]
    fn map_with_capacity_constructors() {
        let m1: AxIndexMap<u32, u32> = AxIndexMap::with_capacity(64);
        let m2: AxIndexMap<u32, u32> = map_with_capacity(64);
        assert!(m1.capacity() >= 64 && m2.capacity() >= 64);
    }

    #[test]
    fn map_seeded_hasher() {
        let mut map: AxIndexMap<&str, i32, AxBuildHasher> =
            AxIndexMap::with_hasher(AxBuildHasher::with_seed(0xc0ffee_deadbeef));
        map.insert("seeded", -1);
        assert_eq!(map["seeded"], -1);
    }

    #[test]
    fn map_insertion_order_preserved() {
        let mut map: AxIndexMap<&str, u32> = AxIndexMap::new();
        let words = ["delta", "alpha", "charlie", "bravo"];
        for (i, w) in words.iter().enumerate() {
            map.insert(w, i as u32);
        }
        assert_eq!(map.keys().copied().collect::<Vec<_>>(), words);
    }

    #[test]
    fn map_index_based_access() {
        let mut map: AxIndexMap<&str, u32> = AxIndexMap::new();
        map.insert("first", 1);
        map.insert("second", 2);
        map.insert("third", 3);
        assert_eq!(map.get_index(0), Some((&"first", &1)));
        assert_eq!(map.get_index(2), Some((&"third", &3)));
        assert_eq!(map.get_index(3), None);
    }

    #[test]
    fn map_entry_api() {
        let mut map: AxIndexMap<&str, u32> = AxIndexMap::new();
        for _ in 0..3 {
            map.entry("counter").and_modify(|n| *n += 1).or_insert(0);
        }
        assert_eq!(map["counter"], 2);
    }

    #[test]
    fn map_from_iterator() {
        let map: AxIndexMap<&str, u32> = [("a", 1u32), ("b", 2), ("c", 3)].into_iter().collect();
        assert_eq!(map.len(), 3);
        assert_eq!(map.get_index(0), Some((&"a", &1)));
    }

    #[test]
    fn map_extend() {
        let mut map: AxIndexMap<u32, u32> = AxIndexMap::new();
        map.extend([(1, 10), (2, 20), (3, 30)]);
        assert_eq!(map.len(), 3);
        assert_eq!(map[&2], 20);
    }

    #[test]
    fn map_retain() {
        let mut map: AxIndexMap<u32, u32> = (0u32..10).map(|i| (i, i * i)).collect();
        map.retain(|_, v| *v > 25);
        assert!(map.values().all(|&v| v > 25));
    }

    #[test]
    fn map_sort_keys() {
        let mut map: AxIndexMap<&str, u32> = AxIndexMap::new();
        map.insert("banana", 2);
        map.insert("apple", 1);
        map.insert("cherry", 3);
        map.sort_keys();
        assert_eq!(
            map.keys().copied().collect::<Vec<_>>(),
            ["apple", "banana", "cherry"]
        );
    }

    #[test]
    fn map_swap_remove() {
        let mut map: AxIndexMap<u32, u32> = (0u32..5).map(|i| (i, i)).collect();
        map.swap_remove(&2);
        assert_eq!(map.len(), 4);
        assert!(!map.contains_key(&2));
    }

    #[test]
    fn map_into_inner_roundtrip() {
        let mut raw: RawIndexMap<&str, i32, BuildHasherDefault<AxHasher>> =
            RawIndexMap::with_hasher(BuildHasherDefault::default());
        raw.insert("x", 99);
        let ax: AxIndexMap<&str, i32> = raw.into();
        assert_eq!(ax["x"], 99);
        let raw: RawIndexMap<&str, i32, BuildHasherDefault<AxHasher>> = ax.into();
        assert_eq!(raw["x"], 99);
    }

    #[test]
    fn alias_indexmap_basic() {
        let mut map: IndexMap<&str, u32> = RawIndexMap::with_hasher(BuildHasherDefault::default());
        map.insert("hello", 42);
        assert_eq!(map["hello"], 42);
        // Insertion order preserved.
        map.insert("world", 7);
        assert_eq!(map.get_index(0), Some((&"hello", &42)));
    }

    #[test]
    fn alias_indexmap_collect() {
        let map: IndexMap<&str, u32> = [("a", 1u32), ("b", 2), ("c", 3)]
            .into_iter()
            .collect::<RawIndexMap<_, _, BuildHasherDefault<AxHasher>>>();
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn set_constructors() {
        let s1: AxIndexSet<u32> = AxIndexSet::default();
        let s2: AxIndexSet<u32> = AxIndexSet::new();
        let s3: AxIndexSet<u32> = new_set();
        let s4: AxIndexSet<u32> = AxIndexSet::with_hasher(BuildHasherDefault::default());
        assert!(s1.is_empty() && s2.is_empty() && s3.is_empty() && s4.is_empty());
    }

    #[test]
    fn set_with_capacity_constructors() {
        let s1: AxIndexSet<u64> = AxIndexSet::with_capacity(128);
        let s2: AxIndexSet<u64> = set_with_capacity(128);
        assert!(s1.capacity() >= 128 && s2.capacity() >= 128);
    }

    #[test]
    fn set_insertion_order_and_dedup() {
        let mut set: AxIndexSet<u32> = AxIndexSet::new();
        for n in [5, 3, 1, 3, 5, 2, 4, 2] {
            set.insert(n);
        }
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), [5, 3, 1, 2, 4]);
    }

    #[test]
    fn set_index_based_access() {
        let set: AxIndexSet<&str> = ["x", "y", "z"].into_iter().collect();
        assert_eq!(set.get_index(0), Some(&"x"));
        assert_eq!(set.get_index(2), Some(&"z"));
    }

    #[test]
    fn set_operations() {
        let a: AxIndexSet<u32> = [1, 2, 3, 4].into_iter().collect();
        let b: AxIndexSet<u32> = [3, 4, 5, 6].into_iter().collect();
        assert_eq!(a.union(&b).count(), 6);
        assert_eq!(a.intersection(&b).count(), 2);
        assert_eq!(a.difference(&b).count(), 2);
    }

    #[test]
    fn set_from_iterator_dedup() {
        let set: AxIndexSet<i32> = [1, 2, 3, 2, 1, 4].into_iter().collect();
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn set_extend() {
        let mut set: AxIndexSet<u32> = AxIndexSet::new();
        set.extend([1u32, 2, 3]);
        set.extend([3u32, 4, 5]);
        assert_eq!(set.len(), 5);
    }

    #[test]
    fn set_sort() {
        let mut set: AxIndexSet<u32> = [5, 3, 1, 4, 2].into_iter().collect();
        set.sort();
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn set_into_inner_roundtrip() {
        let mut raw: RawIndexSet<u64, BuildHasherDefault<AxHasher>> =
            RawIndexSet::with_hasher(BuildHasherDefault::default());
        raw.insert(42u64);
        let ax: AxIndexSet<u64> = raw.into();
        assert!(ax.contains(&42));
        let raw: RawIndexSet<u64, BuildHasherDefault<AxHasher>> = ax.into();
        assert!(raw.contains(&42));
    }

    #[test]
    fn alias_indexset_basic() {
        let mut set: IndexSet<u32> = RawIndexSet::with_hasher(BuildHasherDefault::default());
        set.insert(1);
        set.insert(2);
        set.insert(2);
        assert_eq!(set.len(), 2);
        assert_eq!(set.get_index(0), Some(&1));
    }
}
