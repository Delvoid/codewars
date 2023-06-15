pub fn order(sentence: &str) -> String {
    let mut words: Vec<&str> = sentence.split_whitespace().collect();
    words.sort_by_key(|word| word.chars().find(|c| c.is_digit(10)).unwrap());

    words.join(" ")
}
