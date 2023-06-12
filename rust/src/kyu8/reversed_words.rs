pub fn reverse_words(words: &str) -> String {
    let reversed: Vec<&str> = words.split_whitespace().rev().collect();
    reversed.join(" ")
}

pub fn reverse_words_2(words: &str) -> String {
    words
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}
