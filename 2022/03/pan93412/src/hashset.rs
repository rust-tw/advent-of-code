//! HashSet utilites for speeding up the construction.

use std::{collections::HashSet, hash::BuildHasher};

pub trait FromArray {
    type Item;

    fn from_array(array: &[Self::Item]) -> Self;
}

impl<T: Eq + std::hash::Hash + Copy, S: Default + BuildHasher> FromArray for HashSet<T, S> {
    type Item = T;

    /// Create [HashSet] from array.
    ///
    /// We can pre-allocate all the space we need thanks to the pre-known length;
    /// therefore, it can gain ≈ 6% performance improvement since we don't need
    /// to re-allocate the space on inserting.
    fn from_array(array: &[Self::Item]) -> Self {
        let mut set = HashSet::with_capacity_and_hasher(array.len(), Default::default());
        set.extend(array.iter());
        set
    }
}
