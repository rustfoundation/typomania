use std::collections::HashMap;

use super::{util, Check, Corpus, Package, Squat};

/// Checks for common typos.
///
/// This is a very flexible check that — to some extent — duplicates functionality found in other
/// checks, but is also somewhat annoying to configure. You may not need this check in your
/// standard set, depending on your threat model.
pub struct Typos {
    typos: HashMap<char, Vec<String>>,
}

impl Typos {
    /// Instantiates a typo check.
    ///
    /// Each element in `typos` is used to rebuild the package name when checking. Each character
    /// will be replaced by each string in the given vector. For example, if the only typo given is
    /// `('a', vec!["bb", "x", ""])`, then a package `apkg` will also be checked agaisnt `bbpkg`,
    /// `xpkg`, and `pkg`.
    pub fn new(typos: impl Iterator<Item = (char, Vec<String>)>) -> Self {
        Self {
            typos: typos.collect(),
        }
    }
}

impl Check for Typos {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();

        for (i, c) in name.chars().enumerate() {
            if let Some(typos) = self.typos.get(&c) {
                for typo in typos.iter() {
                    let name_to_check = util::rebuild_name(name, i, 1, typo);
                    if corpus.possible_squat(&name_to_check, name, package)? {
                        squats.push(Squat::Typo(name_to_check));
                    }
                }
            }
        }

        Ok(squats)
    }
}

#[cfg(test)]
mod tests {
    use crate::checks::testutil::assert_check;

    use super::*;

    #[test]
    fn test_typos() -> crate::Result<()> {
        #[track_caller]
        fn test(input: &str, want: &[&str]) -> crate::Result<()> {
            assert_check(
                Typos::new([('a', vec![String::from("ab"), String::from("b")])].into_iter()),
                input,
                want,
            )
        }

        test("", &[])?;
        test("x", &[])?;
        test("a", &["ab", "b"])?;
        test("xax", &["xabx", "xbx"])?;

        Ok(())
    }
}
