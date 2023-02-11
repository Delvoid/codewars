pub fn words_to_marks(s: &str) -> u32 {
    s.chars().map(|c| c as u32 - 96).sum()
}
