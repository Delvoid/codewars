pub fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(solution("world"), "dlrow");
        assert_eq!(solution("hello"), "olleh");
        assert_eq!(solution(""), "");
        assert_eq!(solution("h"), "h");
    }
}
