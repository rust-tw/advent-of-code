use crate::{InnerChars, compartment::{Compartment, CompartmentPair}, groups::{Groups, Group}, InnerChar};

pub struct Rucksack<'a>(pub InnerChars<'a>);
pub struct Rucksacks<'a>(Vec<Rucksack<'a>>);

impl<'a> From<&'a str> for Rucksacks<'a> {
    fn from(input: &'a str) -> Self {
        let lines = input
            .trim()
            .split('\n')
            .map(|s| Rucksack(s.trim().as_bytes()))
            .collect();

        Self(lines)
    }
}

impl<'a> Rucksack<'a> {
    /// Separate this rucksack to two compartments.
    pub fn to_compartment_pair(&self) -> CompartmentPair<'a> {
        let compartment_split_point = self.0.len().div_floor(2);
        let (left, right) = self.0.split_at(compartment_split_point);

        let left_compartment = Compartment(left);
        let right_compartment = Compartment(right);

        CompartmentPair((left_compartment, right_compartment))
    }
}

#[delegate_attr::delegate(self.0)]
impl<'a> Rucksack<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &InnerChar>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl<'a> Rucksacks<'a> {
    /// Separate rucksacks to [Iterator] with compartments pair.
    pub fn to_compartment_pairs(&'a self) -> impl Iterator<Item = CompartmentPair<'a>> {
        self.0.iter().map(Rucksack::to_compartment_pair)
    }

    /// Separate rucksacks to groups with specified `LEN`.
    pub fn to_groups<const LEN: usize>(&'a self) -> Groups<'a, LEN> {
        Groups(self.0.array_chunks::<LEN>().map(Group).collect())
    }
}
