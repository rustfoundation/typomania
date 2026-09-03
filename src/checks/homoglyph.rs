use std::collections::HashMap;

use crate::Corpus;

use super::{util, Check, Package, Squat};

/// Checks whether a package only differs from a package in the corpus by substituting visually
/// similar characters.
///
/// This covers both single character confusables (`0` ↔ `o`, `1` ↔ `l`) and multi-character
/// sequences that render similarly to a single glyph (`rn` ↔ `m`, `vv` ↔ `w`).
///
/// Documented attacks include `1odash` targeting `lodash` on npm and `r3quests` targeting
/// `requests` on PyPI.
pub struct Homoglyph {
    glyphs: HashMap<char, Vec<String>>,
    multi: Vec<(String, Vec<String>)>,
}

impl Homoglyph {
    /// Instantiates a homoglyph check with custom substitution tables.
    ///
    /// `glyphs` maps a single character to its visually similar replacements.
    ///
    /// `multi` maps a multi-character sequence to its visually similar replacements. All
    /// occurrences of a multi-character pattern are replaced at once.
    pub fn new(
        glyphs: impl Iterator<Item = (char, Vec<String>)>,
        multi: impl Iterator<Item = (String, Vec<String>)>,
    ) -> Self {
        Self {
            glyphs: glyphs.collect(),
            multi: multi.collect(),
        }
    }
}

impl Default for Homoglyph {
    fn default() -> Self {
        let glyphs = [
            ('a', vec!["4"]),
            ('b', vec!["8", "6"]),
            ('d', vec!["cl"]),
            ('e', vec!["3"]),
            ('g', vec!["9", "6"]),
            ('i', vec!["1", "l"]),
            ('l', vec!["1", "i"]),
            ('m', vec!["rn", "nn"]),
            ('o', vec!["0"]),
            ('s', vec!["5"]),
            ('t', vec!["7"]),
            ('w', vec!["vv", "uu"]),
            ('z', vec!["2"]),
            ('0', vec!["o"]),
            ('1', vec!["l", "i"]),
            ('2', vec!["z"]),
            ('3', vec!["e"]),
            ('4', vec!["a"]),
            ('5', vec!["s"]),
            ('6', vec!["b", "g"]),
            ('7', vec!["t"]),
            ('8', vec!["b"]),
            ('9', vec!["g", "q"]),
        ];

        let multi = [
            ("rn", vec!["m"]),
            ("nn", vec!["m"]),
            ("vv", vec!["w"]),
            ("uu", vec!["w"]),
            ("cl", vec!["d"]),
        ];

        Self::new(
            glyphs
                .into_iter()
                .map(|(c, v)| (c, v.into_iter().map(String::from).collect())),
            multi
                .into_iter()
                .map(|(p, v)| (String::from(p), v.into_iter().map(String::from).collect())),
        )
    }
}

impl Check for Homoglyph {
    fn check(
        &self,
        corpus: &dyn Corpus,
        name: &str,
        package: &dyn Package,
    ) -> crate::Result<Vec<Squat>> {
        let mut squats = Vec::new();
        let mut buf = String::new();

        for (i, c) in name.char_indices() {
            if let Some(glyphs) = self.glyphs.get(&c) {
                for glyph in glyphs.iter() {
                    util::rebuild_name_into(&mut buf, name, i, c.len_utf8(), glyph);
                    if corpus.possible_squat(&buf, name, package)? {
                        squats.push(Squat::Homoglyph(buf.clone()));
                    }
                }
            }
        }

        for (pattern, replacements) in self.multi.iter() {
            if name.contains(pattern.as_str()) {
                for replacement in replacements.iter() {
                    let name_to_check = name.replace(pattern.as_str(), replacement);
                    if corpus.possible_squat(&name_to_check, name, package)? {
                        squats.push(Squat::Homoglyph(name_to_check));
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
            assert_no_panic(Homoglyph::default(), &name);
        }
    }

    #[test]
    fn test_homoglyph() -> crate::Result<()> {
        #[track_caller]
        fn test(input: &str, want: &[&str]) -> crate::Result<()> {
            assert_check(Homoglyph::default(), input, want)
        }

        test("", &[])?;
        test("x", &[])?;
        test("lo", &["1o", "io", "l0"])?;
        test("rn", &["m"])?;
        test("m", &["rn", "nn"])?;
        test("cl", &["d", "c1", "ci"])?;
        test(
            "1odash",
            &["lodash", "iodash", "10dash", "1oclash", "1od4sh", "1oda5h"],
        )?;
        test("élé", &["é1é", "éié"])?;

        Ok(())
    }
}
