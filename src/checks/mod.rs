//! Checks provided by typomania, along with the traits and types required to define custom checks.
//!
//! To implement a custom check, implement the [`Check`] trait, and have it return one or more
//! [`Squat`]s when the package may be squatting one or more packages in the corpus.

use std::fmt::Display;

use crate::{Corpus, Package};

mod bitflips;
mod omitted;
mod repeated;
mod swapped;
mod typos;
mod util;
mod version;

#[cfg(test)]
mod testutil;

pub use bitflips::Bitflips;
pub use omitted::Omitted;
pub use repeated::Repeated;
pub use swapped::{Characters as SwappedCharacters, Words as SwappedWords};
pub use typos::Typos;
pub use version::Version;

/// A check that compares the given package to the existing corpus.
pub trait Check: Sync + Send {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>>;
}

/// A potential typosquat.
#[derive(Debug, Clone)]
pub enum Squat {
    Bitflip(String),
    OmittedCharacter(String),
    RepeatedCharacter(String),
    SwappedCharacters(String),
    SwappedWords(String),
    Typo(String),
    Version(String),
    Custom { message: String, package: String },
}

impl Squat {
    /// Instantiate a custom squat.
    pub fn custom(message: &str, package: &str) -> Self {
        Self::Custom {
            message: message.into(),
            package: package.into(),
        }
    }
}

impl Display for Squat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Squat::Bitflip(package) => write!(f, "may be a bitflip of {package}"),
            Squat::OmittedCharacter(package) => write!(f, "omits characters in {package}"),
            Squat::RepeatedCharacter(package) => write!(f, "repeats characters in {package}"),
            Squat::SwappedCharacters(package) => write!(f, "swaps characters in {package}"),
            Squat::SwappedWords(package) => write!(f, "swaps words in {package}"),
            Squat::Typo(package) => write!(f, "uses a common typo for {package}"),
            Squat::Version(package) => write!(f, "only changes the version from {package}"),
            Squat::Custom { message, package } => write!(f, "{message} for {package}"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{testutil::TestPackage, *};

    struct SimpleCorpus(HashMap<String, TestPackage>);

    impl Corpus for SimpleCorpus {
        fn contains_name(&self, name: &str) -> crate::Result<bool> {
            Ok(self.0.contains_key(name))
        }

        fn get(&self, name: &str) -> crate::Result<Option<&dyn Package>> {
            Ok(if let Some(package) = self.0.get(name) {
                Some(package)
            } else {
                None
            })
        }
    }

    #[test]
    fn test_possible_squat() -> crate::Result<()> {
        let corpus = SimpleCorpus(
            [
                ("a", TestPackage::new("adam")),
                ("d", TestPackage::default()),
            ]
            .into_iter()
            .map(|(name, package)| (String::from(name), package))
            .collect(),
        );

        #[allow(clippy::bool_assert_comparison)]
        {
            // Not a possible squat: same package.
            assert_eq!(
                corpus.possible_squat("a", "a", &TestPackage::default())?,
                false
            );

            // Possible squat: no authors in common. (Even though neither package actually has any
            // authors at all.)
            assert_eq!(
                corpus.possible_squat("d", "x", &TestPackage::default())?,
                true
            );

            // Not a possible squat: author "adam" in common.
            assert_eq!(
                corpus.possible_squat("a", "x", &TestPackage::new("adam"))?,
                false
            );

            // Possible squat: no authors in common.
            assert_eq!(
                corpus.possible_squat("a", "x", &TestPackage::default())?,
                true
            );
        }

        Ok(())
    }
}
