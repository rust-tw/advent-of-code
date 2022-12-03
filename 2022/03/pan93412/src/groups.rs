use std::collections::HashSet;

use crate::{rucksack::Rucksack, item::Item, intersection::IntersectionExt, hashset::FromArray};

/// The rucksacks group.
///
/// A group should contains exactly `LEN` rucksacks.
pub struct Group<'a, const LEN: usize>(pub(crate) &'a [Rucksack<'a>; LEN]);

/// The lists of rucksacks groups.
pub struct Groups<'a, const LEN: usize>(pub Vec<Group<'a, LEN>>);

impl<'a> Group<'a, 3> {
    /// Find the common item between the rucksacks.
    pub fn find_common_item(&self) -> Option<Item> {
        let [g1, g2, g3] = [&self.0[0], &self.0[1], &self.0[2]];
        let g1 = HashSet::from_array(g1.0);
        let g2 = HashSet::from_array(g2.0);
        let g3 = HashSet::from_array(g3.0);

        let mut intersection = g1.intersection(&g2).intersection(&g3);
        intersection.next().map(|c| Item(*c))
    }
}

#[delegate_attr::delegate(self.0)]
impl<'a, const LEN: usize> Groups<'a, LEN> {
    pub fn iter(&self) -> impl Iterator<Item = &Group<'a, LEN>>;
}
