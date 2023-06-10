pub fn number(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{}: {}", i + 1, line))
        .collect()
}
