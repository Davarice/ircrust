pub fn split_at_first(line: &str, _at: char) -> (&str, &str) {
    match line.find(_at) {
        Some(idx) => (&line[..idx], &line[idx + 1..]),
        None => (line, ""),
    }
}
