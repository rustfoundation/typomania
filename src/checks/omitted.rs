use crate::Corpus;

use super::{util, Check, Package, Squat};

/// Checks whether a package only differs from a package in the corpus by omitting one character.
pub struct Omitted {
    alphabet: Vec<String>,
}

impl Omitted {
    /// Instantiates an omitted character check.
    ///
    /// `alphabet` is the list of characters that are valid in a package name.
    pub fn new(alphabet: &str) -> Self {
        Self {
            alphabet: alphabet.chars().map(String::from).collect(),
        }
    }
}

impl Check for Omitted {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();

        for i in 0..=name.len() {
            for c in self.alphabet.iter() {
                let name_to_check = util::rebuild_name(name, i, 0, c);
                if corpus.possible_squat(&name_to_check, name, package)? {
                    squats.push(Squat::OmittedCharacter(name_to_check));
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
    fn test_omitted() -> crate::Result<()> {
        assert_check(
            Omitted::new("abc"),
            "xyz",
            &[
                "axyz", "bxyz", "cxyz", "xayz", "xbyz", "xcyz", "xyaz", "xybz", "xycz", "xyza",
                "xyzb", "xyzc",
            ],
        )
    }
}
