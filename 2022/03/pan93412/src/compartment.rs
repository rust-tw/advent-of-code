use std::collections::{HashSet, hash_map::RandomState};

use crate::{InnerChars, item::Item, hashset::FromArray};

pub struct Compartment<'a>(pub(crate) InnerChars<'a>);

/// The pair (tuple with 2 elements) of compartments.
///
/// This *newtype* pattern is for adding some useful methods,
/// such as [CompartmentPair::find_common_item].
///
/// `ASSURE` ensures this pair has the specified trait.
pub struct CompartmentPair<'a>(pub (Compartment<'a>, Compartment<'a>));

impl<'a> CompartmentPair<'a> {
    /// Find the common item between two compartments.
    pub fn find_common_item(&self) -> Option<Item> {
        let (left, right) = &self.0;
        let left_chars = HashSet::<_, RandomState>::from_array(left.0);
        let right_chars = HashSet::<_, RandomState>::from_array(right.0);

        let mut common_char = left_chars.intersection(&right_chars);
        common_char.next().map(|c| Item(*c))
    }
}
