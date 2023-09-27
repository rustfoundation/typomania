use std::marker::PhantomData;

use itertools::Itertools;
use thiserror::Error;
use tracing::instrument;

use crate::{
    checks::{Check, Repeated, Squat, SwappedCharacters, Version},
    BoxError, Corpus, Package,
};

/// A basic harness that runs its configured checks against one or more potentially typosquatted
/// packages.
///
/// If the `rayon` feature is enabled, the [`Harness::check`] method can be used to check many
/// packages in parallel, using Rayon for parallelisation.
pub struct Harness<C>
where
    C: Corpus + Send + Sync,
{
    checks: Vec<Box<dyn Check>>,
    corpus: C,
}

/// A builder for [`Harness`].
pub struct Builder<C>
where
    C: Corpus + Send + Sync,
{
    checks: Vec<Box<dyn Check>>,
    _marker: PhantomData<C>,
}

impl<C> Builder<C>
where
    C: Corpus + Send + Sync,
{
    fn new() -> Self {
        let repeated: Box<dyn Check> = Box::new(Repeated);
        let swapped_chars: Box<dyn Check> = Box::new(SwappedCharacters);
        let version: Box<dyn Check> = Box::new(Version);

        Self {
            checks: Vec::from([repeated, swapped_chars, version]),
            _marker: PhantomData,
        }
    }

    fn empty() -> Self {
        Self {
            checks: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Adds a check to the harness.
    pub fn with_check<Chk>(mut self, check: Chk) -> Self
    where
        Chk: Check + 'static,
    {
        self.checks.push(Box::new(check));
        self
    }

    /// Uses the given corpus to build a harness.
    pub fn build(self, corpus: C) -> Harness<C>
    where
        C: Corpus + Send + Sync + 'static,
    {
        Harness {
            checks: self.checks,
            corpus,
        }
    }
}

impl<C> Harness<C>
where
    C: Corpus + Send + Sync + 'static,
{
    /// Instantiates a builder with three checks configured by default: [`Repeated`],
    /// [`SwappedCharacters`], and [`Version`].
    ///
    /// These checks are provided by default because they don't require any specific knowledge of
    /// the package ecosystem.
    pub fn builder() -> Builder<C> {
        Builder::new()
    }

    /// Instantiates a builder with no checks.
    pub fn empty_builder() -> Builder<C> {
        Builder::empty()
    }

    /// Checks all given packages against the corpus, using Rayon to parallelise the checks.
    #[cfg(feature = "rayon")]
    #[instrument(level = "DEBUG", skip_all, err)]
    pub fn check(
        &self,
        new_packages: impl Iterator<Item = (String, Box<dyn Package>)> + Send,
    ) -> Result<std::collections::HashMap<String, Vec<Squat>>, Error> {
        use rayon::prelude::*;

        new_packages
            .par_bridge()
            .into_par_iter()
            .filter_map(|(name, package)| match self.check_package(&name, package) {
                Ok(squats) if squats.is_empty() => None,
                Ok(squats) => Some(Ok((name, squats))),
                Err(e) => Some(Err(e)),
            })
            .collect()
    }

    /// Checks a single package against the corpus using the configured checks.
    #[instrument(level = "TRACE", skip(self, package), err)]
    pub fn check_package(
        &self,
        name: &str,
        package: Box<dyn Package>,
    ) -> Result<Vec<Squat>, Error> {
        if self.corpus.contains_name(name)? {
            return Ok(Vec::new());
        }

        self.checks
            .iter()
            .map(|check| -> Result<Vec<Squat>, Error> {
                Ok(check.check(&self.corpus, name, package.as_ref())?)
            })
            .flatten_ok()
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("corpus error: {0}")]
    Corpus(String),
}

impl From<BoxError> for Error {
    fn from(value: BoxError) -> Self {
        Self::Corpus(value.to_string())
    }
}
