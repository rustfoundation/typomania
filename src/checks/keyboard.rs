use std::collections::HashMap;

use crate::Corpus;

use super::{util, Check, Package, Squat};

/// Checks whether a package only differs from a package in the corpus by replacing one character
/// with an adjacent key on a keyboard.
///
/// This is distinct from [`super::Typos`], which targets curated misspellings: this check
/// systematically generates every single-character replacement based on physical key proximity.
///
/// Documented attacks include `requezts` and `requeats` targeting `requests` on PyPI.
pub struct KeyboardAdjacent {
    adjacent: HashMap<char, Vec<String>>,
}

impl KeyboardAdjacent {
    /// Instantiates a keyboard-adjacent check with a custom layout.
    ///
    /// Each entry maps a key to the keys physically surrounding it.
    pub fn new(adjacent: impl Iterator<Item = (char, Vec<String>)>) -> Self {
        Self {
            adjacent: adjacent.collect(),
        }
    }

    /// Instantiates a keyboard-adjacent check using a US QWERTY layout.
    pub fn qwerty() -> Self {
        let layout = [
            ('q', vec!["w", "a", "s"]),
            ('w', vec!["q", "e", "a", "s", "d"]),
            ('e', vec!["w", "r", "s", "d", "f"]),
            ('r', vec!["e", "t", "d", "f", "g"]),
            ('t', vec!["r", "y", "f", "g", "h"]),
            ('y', vec!["t", "u", "g", "h", "j"]),
            ('u', vec!["y", "i", "h", "j", "k"]),
            ('i', vec!["u", "o", "j", "k", "l"]),
            ('o', vec!["i", "p", "k", "l"]),
            ('p', vec!["o", "l"]),
            ('a', vec!["q", "w", "s", "z"]),
            ('s', vec!["q", "w", "e", "a", "d", "z", "x"]),
            ('d', vec!["w", "e", "r", "s", "f", "x", "c"]),
            ('f', vec!["e", "r", "t", "d", "g", "c", "v"]),
            ('g', vec!["r", "t", "y", "f", "h", "v", "b"]),
            ('h', vec!["t", "y", "u", "g", "j", "b", "n"]),
            ('j', vec!["y", "u", "i", "h", "k", "n", "m"]),
            ('k', vec!["u", "i", "o", "j", "l", "m"]),
            ('l', vec!["i", "o", "p", "k"]),
            ('z', vec!["a", "s", "x"]),
            ('x', vec!["s", "d", "z", "c"]),
            ('c', vec!["d", "f", "x", "v"]),
            ('v', vec!["f", "g", "c", "b"]),
            ('b', vec!["g", "h", "v", "n"]),
            ('n', vec!["h", "j", "b", "m"]),
            ('m', vec!["j", "k", "n"]),
            ('1', vec!["2", "q"]),
            ('2', vec!["1", "3", "q", "w"]),
            ('3', vec!["2", "4", "w", "e"]),
            ('4', vec!["3", "5", "e", "r"]),
            ('5', vec!["4", "6", "r", "t"]),
            ('6', vec!["5", "7", "t", "y"]),
            ('7', vec!["6", "8", "y", "u"]),
            ('8', vec!["7", "9", "u", "i"]),
            ('9', vec!["8", "0", "i", "o"]),
            ('0', vec!["9", "o", "p"]),
        ];

        Self::new(
            layout
                .into_iter()
                .map(|(c, v)| (c, v.into_iter().map(String::from).collect())),
        )
    }
}

impl Default for KeyboardAdjacent {
    fn default() -> Self {
        Self::qwerty()
    }
}

impl Check for KeyboardAdjacent {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();
        let mut buf = String::new();

        for (i, c) in name.char_indices() {
            if let Some(keys) = self.adjacent.get(&c) {
                for key in keys.iter() {
                    util::rebuild_name_into(&mut buf, name, i, c.len_utf8(), key);
                    if corpus.possible_squat(&buf, name, package)? {
                        squats.push(Squat::KeyboardAdjacent(buf.clone()));
                    }
                }
            }
        }

        Ok(squats)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::checks::testutil::{assert_check, assert_no_panic, name_strategy};

    use super::*;

    proptest! {
        #[test]
        fn never_panics(name in name_strategy()) {
            assert_no_panic(KeyboardAdjacent::qwerty(), &name);
        }
    }

    #[test]
    fn test_keyboard_adjacent() -> crate::Result<()> {
        #[track_caller]
        fn test(input: &str, want: &[&str]) -> crate::Result<()> {
            assert_check(KeyboardAdjacent::qwerty(), input, want)
        }

        test("", &[])?;
        test("-", &[])?;
        test("p", &["o", "l"])?;
        test("qz", &["wz", "az", "sz", "qa", "qs", "qx"])?;
        test(
            "ts",
            &[
                "rs", "ys", "fs", "gs", "hs", "tq", "tw", "te", "ta", "td", "tz", "tx",
            ],
        )?;
        test("épé", &["éoé", "élé"])?;

        Ok(())
    }
}
