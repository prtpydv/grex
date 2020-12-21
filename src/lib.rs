/// Given an iterator on strings and a pattern string to search, returns an
/// iterator of all the strings which contain the pattern.
pub fn find_matches<'a>(
    lines: impl Iterator<Item = String> + 'a,
    pattern: &'a str,
) -> impl Iterator<Item = String> + 'a {
    lines.filter(move |line| line.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::find_matches;

    #[test]
    fn test_find_matches() {
        let lines = ["A test", "Actual content", "More content", "Another test"]
            .iter()
            .map(|s| s.to_string());
        let results: Vec<_> = find_matches(lines, "test").collect();
        assert_eq!(results, ["A test", "Another test"]);
    }
}