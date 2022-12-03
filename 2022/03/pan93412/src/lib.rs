#![feature(int_roundings)]
#![feature(array_chunks)]

pub mod hashset;
pub mod intersection;
pub mod compartment;
pub mod rucksack;
pub mod groups;
pub mod item;
pub mod solution;

/// The inner data structure of the deserialized data.
///
/// I want to be `zero-copy` as much as possible; therefore,
/// I use `[u8]` to hold our inputs, because:
///
/// - No Unicode characters needed in this case.
/// - We can easily split data into any length of parts,
///   which is not easy (with `zero-copy` mission) for iterators.
///
/// The drawback is the “lifetime”. However, since we don't own our data,
/// we don't need to care too much about it. You can change to `Cow<'a, [u8]>`
/// if you need to own the data.
type InnerChars<'a> = &'a [InnerChar];
type InnerChar = u8;
