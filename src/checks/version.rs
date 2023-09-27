use super::{Check, Corpus, Package, Squat};

/// Checks whether a package only differs from a package in the corpus by omitting a version
/// number.
pub struct Version;

impl Check for Version {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let trimmed = name
            .trim_end_matches(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
            .trim_end_matches('-');

        Ok(
            if !trimmed.is_empty()
                && trimmed != name
                && corpus.possible_squat(trimmed, name, package)?
            {
                vec![Squat::Version(trimmed.into())]
            } else {
                Vec::new()
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::checks::testutil::assert_check;

    use super::*;

    #[test]
    fn test_version() -> crate::Result<()> {
        assert_check(Version, "", &[])?;
        assert_check(Version, "-2", &[])?;
        assert_check(Version, "2", &[])?;
        assert_check(Version, "abc", &[])?;
        assert_check(Version, "abc234", &["abc"])?;
        assert_check(Version, "abc-234", &["abc"])?;
        assert_check(Version, "abc-", &["abc"])?;
        assert_check(Version, "abc0", &["abc"])?;

        Ok(())
    }
}
