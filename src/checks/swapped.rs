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
}

impl Words {
    /// Sets up a swapped word check, using each character in `delimiters` as a possible word
    /// delimiter.
    pub fn new(delimiters: &str) -> Self {
        Self {
            delimiters: delimiters.chars().collect(),
        }
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

        for case in tokens.into_iter().permutations(num_tokens) {
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

        Ok(())
    }
}
