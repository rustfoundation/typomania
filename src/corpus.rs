//! The [`Corpus`] trait, and utility functions related to implementing it.

use crate::{Package, Result};

/// A corpus of existing, popular packages that checks must be run against.
///
/// This is implemented by default for `HashMap<String, Package>` and `BTreeMap<String, Package>`.
/// Users with more complex needs can adapt their own package sources, provided they return
/// [`Package`].
pub trait Corpus: Send + Sync {
    fn contains_name(&self, name: &str) -> Result<bool>;
    fn get(&self, name: &str) -> Result<Option<&dyn Package>>;

    /// Checks if `corpus_name` — a package in the corpus — should be considered to be squatting
    /// package `package`, identified by `package_name`.
    ///
    /// This can be used to implement filters based on ecosystem-specific knowledge that isn't
    /// exposed in the generic [`Package`] trait.
    ///
    /// The default implementation is [`default_possible_squat`]. Implementors replacing the
    /// default implementation may still want to invoke [`default_possible_squat`] before adding
    /// their own filtering.
    fn possible_squat(
        &self,
        corpus_name: &str,
        package_name: &str,
        package: &dyn Package,
    ) -> Result<bool> {
        default_possible_squat(self, corpus_name, package_name, package)
    }
}

/// The default implementation of [`Corpus::possible_squat`], split out for easier reuse in other
/// [`Corpus`] implementations.
///
/// This implementation checks two things:
///
/// 1. Is `corpus_name` the same as `package_name`?
/// 1. Does the package in the corpus share any authors with `package`?
///
/// If either of these checks returns true, then this function returns `false`, as it's assumed
/// that a package cannot squat itself, and that an author cannot squat their own package.
pub fn default_possible_squat<C>(
    corpus: &C,
    corpus_name: &str,
    package_name: &str,
    package: &dyn Package,
) -> Result<bool>
where
    C: Corpus + Send + Sync + ?Sized,
{
    Ok(if corpus_name == package_name {
        // The same package can't squat itself.
        false
    } else if let Some(checked) = corpus.get(corpus_name)? {
        // See if there are any shared authors. If not, then this might be squatted.
        !checked.shared_authors(package.authors())
    } else {
        false
    })
}
