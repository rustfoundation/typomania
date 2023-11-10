use itertools::Itertools;

use super::{util, Check, Corpus, Package, Squat};

/// Checks whether one or more characters have been swapped in the given package name.
pub struct Characters;

impl Check for Characters {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();

        for (i, (a, b)) in name.chars().tuple_windows().enumerate() {
            if a != b {
                let name_to_check = util::rebuild_name(name, i, 2, &format!("{b}{a}"));
                if corpus.possible_squat(&name_to_check, name, package)? {
                    squats.push(Squat::SwappedCharacters(name_to_check));
                }
            }
        }

        Ok(squats)
    }
}

/// Checks whether one or more words have been swapped in the given package name.
pub struct Words {
    delimiters: Vec<char>,
    max_k: usize,
}

impl Words {
    /// Sets up a swapped word check, using each character in `delimiters` as a possible word
    /// delimiter, and a max `k` of 5 (see `[Words::with_max_k]` for more detail).
    pub fn new(delimiters: &str) -> Self {
        Self {
            delimiters: delimiters.chars().collect(),
            max_k: 5,
        }
    }

    /// Changes the maximum value of k when calculating the k-permutations of the parts of the
    /// package name.
    ///
    /// What this practically means is that packages with values beyond the maximum will only have
    /// partial permutations checked; for example, if the limit is 3 and a package
    /// `foo-bar-baz-quux` is checked, the only package names that will be checked are the three
    /// element permutations (eg `foo-bar-baz`, `foo-bar-quux`, etc), not permutations of the full
    /// set.
    ///
    /// This is provided to avoid inadvertent DoS issues. For offline analysis, you may want to set
    /// `max_k` to a large value, provided you have sufficient RAM.
    pub fn with_max_k(mut self, max_k: usize) -> Self {
        self.max_k = max_k;
        self
    }
}

impl Check for Words {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();

        let tokens: Vec<String> = name
            .split(self.delimiters.as_slice())
            .map(String::from)
            .collect();

        // Short circuit if there's still only one token.
        let num_tokens = tokens.len();
        if num_tokens == 1 {
            return Ok(squats);
        }

        // Apply the max_k.
        let k = if num_tokens > self.max_k {
            self.max_k
        } else {
            num_tokens
        };

        for case in tokens.into_iter().permutations(k) {
            for delimiter in self.delimiters.iter() {
                let name_to_check = case.join(&format!("{delimiter}"));
                if corpus.possible_squat(&name_to_check, name, package)? {
                    squats.push(Squat::SwappedWords(name_to_check));
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
    fn test_characters() -> crate::Result<()> {
        #[track_caller]
        fn test(input: &str, want: &[&str]) -> crate::Result<()> {
            assert_check(Characters, input, want)
        }

        test("", &[])?;
        test("a", &[])?;
        test("ab", &["ba"])?;
        test("abc", &["bac", "acb"])?;

        Ok(())
    }

    #[test]
    fn test_words() -> crate::Result<()> {
        #[track_caller]
        fn test(input: &str, want: &[&str]) -> crate::Result<()> {
            assert_check(Words::new("-_"), input, want)
        }

        test("", &[])?;
        test("a", &[])?;
        test("abc", &[])?;
        test("abc-def", &["abc_def", "def-abc", "def_abc"])?;
        test(
            "abc-def_ghi",
            &[
                "abc_def_ghi",
                "abc-def-ghi",
                "abc_ghi_def",
                "abc-ghi-def",
                "def_abc_ghi",
                "def-abc-ghi",
                "def_ghi_abc",
                "def-ghi-abc",
                "ghi_abc_def",
                "ghi-abc-def",
                "ghi_def_abc",
                "ghi-def-abc",
            ],
        )?;

        // Test max_k.
        assert_check(
            Words::new("-_").with_max_k(2),
            "a-b-c",
            &[
                "b_a", "c_b", "c-b", "a-c", "c_a", "b-a", "c-a", "a_c", "b_c", "a-b", "b-c", "a_b",
            ],
        )?;

        Ok(())
    }
}
