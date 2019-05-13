pub fn split_at_first(_line: &str, _at: char) -> [&str; 2] {
    let one: &str;
    let two: &str;
    if _line.contains(_at) {
        let idx: usize = _line.find(_at).unwrap();
        one = &_line[..idx];
        two = &_line[idx + 1..];
    } else {
        one = _line;
        two = "";
    }
    [one, two]
}
