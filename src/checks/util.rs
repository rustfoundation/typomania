pub(super) fn rebuild_name(orig: &str, index: usize, replace: usize, replacement: &str) -> String {
    format!(
        "{before}{replacement}{after}",
        before = &orig[0..index],
        after = if let Some(after) = orig.get(index + replace..) {
            after
        } else {
            ""
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rebuild_name() {
        assert_eq!("foobar", rebuild_name("foobar", 3, 0, ""));
        assert_eq!("fooxbar", rebuild_name("foobar", 3, 0, "x"));
        assert_eq!("fooxar", rebuild_name("foobar", 3, 1, "x"));
        assert_eq!("fxbar", rebuild_name("foobar", 1, 2, "x"));
        assert_eq!("fxxbar", rebuild_name("foobar", 1, 2, "xx"));
    }
}
