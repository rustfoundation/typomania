use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;
use tracing::instrument;

use crate::{BoxError, Corpus, Package};

use super::{Check, Squat};

/// Checks whether the package is a bitflipped version of a package in the corpus.
///
/// This attempts to detect [bitsquatting attacks][bitsquatting].
///
/// [bitsquatting]: https://en.wikipedia.org/wiki/Bitsquatting
pub struct Bitflips {
    bitflips: BTreeMap<String, Vec<usize>>,
    names: Vec<String>,
}

impl Bitflips {
    /// Instantiates a bitflip check.
    ///
    /// `alphabet` is the list of characters that are valid in a package name.
    ///
    /// `names` is generally the same set of names that exist in the top package corpus: a local
    /// copy is required so that the list of possible bitflips can be generated during
    /// instantiation, rather than having to recalculate the list each time the check is run.
    #[instrument(level = "TRACE", skip(names))]
    pub fn new<'a>(alphabet: &str, names: impl Iterator<Item = &'a str>) -> Self {
        let alphabet: BTreeSet<char> = alphabet.chars().collect();
        let mut bitflips: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        let mut cloned_names = Vec::new();

        for (i, name) in names.enumerate() {
            cloned_names.push(name.into());
            for bitflipped_name in
                bitflip::ascii_str(name).filter(|bf| bf.chars().all(|c| alphabet.contains(&c)))
            {
                bitflips.entry(bitflipped_name).or_default().push(i);
            }
        }

        Self {
            bitflips,
            names: cloned_names,
        }
    }
}

impl Check for Bitflips {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();

        if let Some(indices) = self.bitflips.get(name) {
            for index in indices.iter().copied() {
                let name_to_check = self.names.get(index).ok_or(Error::OutOfRangeIndex {
                    index,
                    len: self.names.len(),
                })?;
                if corpus.possible_squat(name_to_check, name, package)? {
                    squats.push(Squat::Bitflip(name_to_check.into()))
                }
            }
        }

        Ok(squats)
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Corpus(#[from] BoxError),

    #[error("unexpected out of range index {index} in vec of length {len}")]
    OutOfRangeIndex { index: usize, len: usize },
}

#[cfg(test)]
mod tests {
    use crate::checks::testutil::assert_check;

    use super::*;

    #[test]
    fn test_bitflips() -> crate::Result<()> {
        assert_check(Bitflips::new("abcdef", ["ab"].into_iter()), "ac", &["ab"])?;

        // Even more limited alphabet.
        assert_check(Bitflips::new("ab", ["ab"].into_iter()), "ac", &[])
    }
}
