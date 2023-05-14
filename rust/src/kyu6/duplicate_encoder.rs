use std::collections::HashMap;

pub fn duplicate_encode(word: &str) -> String {
    let mut map = HashMap::new();
    for c in word.to_ascii_lowercase().chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }
    word.to_ascii_lowercase()
        .chars()
        .map(|c| if map[&c] > 1 { ')' } else { '(' })
        .collect()
}
