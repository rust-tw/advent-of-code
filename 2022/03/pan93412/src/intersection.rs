use std::{
    collections::{hash_set::Intersection, HashSet, hash_map::RandomState},
    hash::Hash,
};

/// The extension method for [Intersection].
pub trait IntersectionExt<'a> {
    type Item: Hash;
    type State;

    /// Get the intersection of another [HashSet].
    fn intersection(
        self,
        other: &'a HashSet<Self::Item>,
    ) -> ThreeWayIntersection<'a, Self::Item, Self::State>;
}

impl<'a, T: Hash, S> IntersectionExt<'a> for Intersection<'a, T, S> {
    type Item = T;
    type State = S;

    fn intersection(
        self,
        other: &'a HashSet<Self::Item>,
    ) -> ThreeWayIntersection<'a, Self::Item, S> {
        ThreeWayIntersection::new(self, other)
    }
}

/// Get the intersection of three [HashSet].
pub struct ThreeWayIntersection<'a, T: Hash, S> {
    base: Intersection<'a, T, S>,
    next: &'a HashSet<T>,
}

impl<'a, T: Hash, S> ThreeWayIntersection<'a, T, S> {
    pub fn new(base: Intersection<'a, T, S>, next: &'a HashSet<T>) -> Self {
        Self { base, next }
    }
}

impl<'a, T: Hash + Eq> Iterator for ThreeWayIntersection<'a, T, RandomState> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.find(|&item| self.next.contains(item))
    }
}
