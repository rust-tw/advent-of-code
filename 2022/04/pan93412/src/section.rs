use hashbrown::HashSet;

pub type IdNumber = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct Section(HashSet<IdNumber>);

impl FromIterator<IdNumber> for Section {
    fn from_iter<T: IntoIterator<Item = IdNumber>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Section {
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

        Ok(Self::from_iter(start..=end))
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.is_disjoint(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::section::Section;

    #[test]
    fn test_from_section_string() {
        let section = Section::from_section_string("1-5").unwrap();
        assert_eq!(section.0, [1, 2, 3, 4, 5].into_iter().collect());
    }
}
