pub fn reverse_words(str: &str) -> String {
    let words: Vec<String> = str
        .split_whitespace()
        .filter(|word| !word.len() > 0)
        .map(|word| word.chars().rev().collect::<String>())
        .collect();
    words.join(" ")
}

pub fn reverse_word_2(str: &str) -> String {
    let words: Vec<String> = str
        .split(" ")
        .filter(|word| !word.len() > 0)
        .map(|word| word.chars().rev().collect::<String>())
        .collect();
    words.join(" ")
}
