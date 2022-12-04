use std::io::BufRead;

use crate::section::Section;

#[derive(Debug, PartialEq, Eq)]
pub struct Assignment((Section, Section));

#[derive(Debug, PartialEq, Eq)]
pub struct Assignments(Vec<Assignment>);

impl Assignment {
    /// Construct a [Assignment] from a section string like `12-24,23-36`
    pub fn from_assignment_string(assignment: &str) -> anyhow::Result<Self> {
        let mut split = assignment.split(',');

        let section_1 = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing section 1: {assignment}"))?;
        let section_2 = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing section 2: {assignment}"))?;

        Ok(Assignment((
            Section::from_section_string(section_1)?,
            Section::from_section_string(section_2)?,
        )))
    }

    /// Does one section fully contain the other?
    pub fn has_subset(&self) -> bool {
        let (section_1, section_2) = &self.0;
        section_1.is_subset(section_2) || section_2.is_subset(section_1)
    }

    /// Does these sections has any overlap part?
    pub fn has_overlap(&self) -> bool {
        let (section_1, section_2) = &self.0;
        !section_1.is_disjoint(section_2) || !section_2.is_disjoint(section_1)
    }
}

impl Assignments {
    /// Construct a [Assignments] from lines of section strings.
    ///
    /// See `Assignment::from_assignment_string`.
    pub fn from_assignments_string(assignments: &str) -> anyhow::Result<Self> {
        let assignments = assignments
            .lines()
            .map(Assignment::from_assignment_string)
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self(assignments))
    }

    /// Construct a [Assignments] from readers implementing [Read].
    pub fn from_reader<R: BufRead>(reader: R) -> anyhow::Result<Self> {
        let assignments = reader
            .lines()
            .map(|s| Assignment::from_assignment_string(&s?))
            .collect::<anyhow::Result<Vec<_>>>();

        Ok(Self(assignments?))
    }

    /// Count the number of assignments that have a subset.
    pub fn count_subset(&self) -> usize {
        self.0.iter().filter(|a| a.has_subset()).count()
    }

    /// Count the number of assignments that have an overlap.
    pub fn count_overlap(&self) -> usize {
        self.0.iter().filter(|a| a.has_overlap()).count()
    }
}

#[cfg(test)]
mod tests {
    const PRINT_DEBUG_LOG: bool = false;
    use super::Assignment;

    #[test]
    fn test_from_assignment_string() {
        let assignment = Assignment::from_assignment_string("12-24,23-36").unwrap();
        let expected_left_section = super::Section::from_iter(12..=24);
        let expected_right_section = super::Section::from_iter(23..=36);

        assert_eq!(
            assignment,
            Assignment((expected_left_section, expected_right_section))
        );
    }

    #[test]
    fn test_has_subset() {
        let testsets = [
            ("12-24,23-36", false),
            ("12-24,25-36", false),
            ("6-21,12-22", false),
            ("12-24,12-24", true),
            ("12-24,18-22", true),
            ("12-24,12-18", true),
            ("12-24,18-24", true),
        ];

        for (index, (astr, has_subset)) in testsets.into_iter().enumerate() {
            let assignment = Assignment::from_assignment_string(astr).unwrap();
            if PRINT_DEBUG_LOG {
                eprintln!(
                    "[subset] test: {assignment:?}, at idx: {index}",
                    index = index + 1
                )
            }
            assert_eq!(assignment.has_subset(), has_subset);
        }
    }

    #[test]
    fn test_has_overlap() {
        let testsets = [
            ("12-24,25-36", false),
            ("12-23,35-38", false),
            ("6-12,1-4", false),
            ("12-24,23-36", true),
            ("6-21,12-22", true),
            ("12-24,12-24", true),
            ("12-24,18-22", true),
            ("12-24,12-18", true),
            ("12-24,18-24", true),
        ];

        for (index, (astr, has_subset)) in testsets.into_iter().enumerate() {
            let assignment = Assignment::from_assignment_string(astr).unwrap();
            if PRINT_DEBUG_LOG {
                eprintln!(
                    "[overlap] test: {assignment:?}, at idx: {index}",
                    index = index + 1
                )
            }
            assert_eq!(assignment.has_overlap(), has_subset);
        }
    }
}
