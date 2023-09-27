/// Common trait that packages must implement to provide common metadata used by checks and
/// corpora.
///
/// Note that "author" is simply a string in this data model. However these are represented, these
/// need to be unique within the package ecosystem: registry user names, user IDs, or e-mail
/// addresses would tend to be reasonable candidates to represent an author.
pub trait Package: Send + Sync {
    /// Returns an object that can be used to check if one or more authors own this package.
    ///
    /// See the documentation for [`AuthorSet`] for more detail, but in most cases, this will be
    /// implemented as:
    ///
    /// ```rust
    /// # use typomania::{AuthorSet, Package};
    /// #
    /// # struct MyPackage;
    /// #
    /// impl Package for MyPackage {
    ///     fn authors(&self) -> &dyn AuthorSet {
    ///         self
    ///     }
    ///
    ///     // ...
    /// #    fn description(&self) -> Option<&str> { unimplemented!() }
    /// #    fn shared_authors(&self, other: &dyn AuthorSet) -> bool { unimplemented!() }
    /// }
    ///
    /// impl AuthorSet for MyPackage {
    ///     fn contains(&self, author: &str) -> bool {
    ///         // ...
    ///         # unimplemented!()
    ///     }
    /// }
    /// ```
    fn authors(&self) -> &dyn AuthorSet;

    /// Returns the package description, if it has one.
    ///
    /// This isn't used by any check shipped by default in typomania, but may be useful for NLP
    /// checks: packages that typosquat others will tend to replicate their descriptions,
    /// summaries, and/or readmes to confuse their targets further.
    fn description(&self) -> Option<&str>;

    /// Checks if any authors on the other [`AuthorSet`] match any authors on this package.
    fn shared_authors(&self, other: &dyn AuthorSet) -> bool;
}

/// Trait that packages must implement to check if they have a particular author.
///
/// In the vast majority of cases, this will be implemented on the same type as [`Package`]: the
/// existence of this as a separate trait is an unfortunate implementation detail.
pub trait AuthorSet {
    fn contains(&self, author: &str) -> bool;
}
