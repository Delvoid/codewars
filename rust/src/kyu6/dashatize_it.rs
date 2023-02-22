pub fn dashatize(num: i64) -> String {
    num.to_string()
        .chars()
        .map(|c| match c {
            '1' | '3' | '5' | '7' | '9' => format!("-{}-", c),
            _ => c.to_string(),
        })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}
