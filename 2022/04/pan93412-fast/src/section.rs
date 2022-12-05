pub type IdNumber = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct Section(IdNumber, IdNumber);

impl FromIterator<IdNumber> for Section {
    fn from_iter<T: IntoIterator<Item = IdNumber>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let first = iter.next().expect("should has 1st element");

        Self::from_bound(first, iter.last().unwrap_or(first))
    }
}

impl Section {
    /// Construct the [Section] from the lower and upper bound.
    pub fn from_bound(lower: IdNumber, upper: IdNumber) -> Self {
        Self::from_bound_unchecked(lower.min(upper), upper.max(lower))
    }

    /// Construct the [Section] from the lower and upper bound
    /// without any checks.
    ///
    /// The `unchecked` version will not check if `lower <= upper`
    /// in release mode.
    pub fn from_bound_unchecked(lower: IdNumber, upper: IdNumber) -> Self {
        debug_assert!(lower <= upper);
        Self(lower, upper)
    }

    /// Construct a [Section] from a section string like `12-24`
    pub fn from_section_string(range: &str) -> anyhow::Result<Self> {
        let mut split = range.split('-');

        let start = split
            .next()
            .map(str::parse)
            .ok_or_else(|| anyhow::anyhow!("Missing start of range: {range}"))??;
        let end = split
            .next()
            .map(str::parse)
            .ok_or_else(|| anyhow::anyhow!("Missing end of range: {range}"))??;

        Ok(Self::from_bound(start, end))
    }

    #[inline(always)]
    fn lower(&self) -> IdNumber {
        self.0
    }

    #[inline(always)]
    fn upper(&self) -> IdNumber {
        self.1
    }

    #[inline]
    pub fn is_subset(&self, other: &Self) -> bool {
        // SELF         |--------------|
        // OTHER        |------|-|-----|
        self.lower() <= other.lower() && self.upper() >= other.upper()
    }

    #[inline]
    pub fn is_overlap(&self, other: &Self) -> bool {
        match ((self.lower(), self.upper()), (other.lower(), other.upper())) {
            // OUR    |----------|
            // THEIR     |------|
            (our, their) if our.0 < their.0 => our.1 >= their.1,
            // OUR           |----------|
            // THEIR  |---------|
            (our, their) if our.0 > their.0 => their.1 >= our.0,
            // our.0 == their.0, so they must be overlapped.
            _ => true,
        }
    }

    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.is_overlap(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::section::Section;

    #[test]
    fn test_from_section_string() {
        let section = Section::from_section_string("1-5").unwrap();
        assert_eq!(section, Section(1, 5));
    }
}
