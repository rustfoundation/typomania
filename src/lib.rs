//! Checks and a harness to detect potential typosquatting in a package repository.
//!
//! This is ported from [`typogard`][typogard], originally by a team led by Matthew Taylor at the
//! University of Kansas and published alongside the [_Defending Against Package
//! Typosquatting_][paper] paper, and adapted by [Dan Gardner][dangardner] for crates.io
//! specifically.
//!
//! ## Theory of operation
//!
//! Given a [`Corpus`] of popular packages, the checks in the [`checks`] module allow new or
//! interesting packages to be matched against that corpus to look for common typosquatting
//! techniques. Custom checks may also be written by implementing [`checks::Check`]; custom checks
//! should use [`checks::Squat::Custom`] when returning potential typosquats.
//!
//! A [`Harness`] is provided that can be used to run a suite of checks against a single package,
//! or — when the `rayon` feature is enabled — against many packages at once in parallel.
//!
//! Checks and corpora both use instances of [`Package`], which provides a basic lowest common
//! denominator representation of ecosystem-specific packages. Users are expected to implement
//! [`Package`] (and the related [`AuthorSet`]) on their native package type for analysis.
//!
//! ## Tracing
//!
//! Potentially expensive operations are traced using `tracing` at the TRACE level, except for
//! [`Harness::check`], which is traced at the DEBUG level.
//!
//! [dangardner]: https://github.com/dangardner/typogard
//! [paper]: https://dl.acm.org/doi/10.1007/978-3-030-65745-1_7
//! [typogard]: https://github.com/mt3443/typogard

pub mod checks;
pub mod corpus;
mod error;
mod harness;
mod package;

pub use corpus::Corpus;
pub use error::{BoxError, Result};
pub use harness::{Builder as HarnessBuilder, Error as HarnessError, Harness};
pub use package::{AuthorSet, Package};
