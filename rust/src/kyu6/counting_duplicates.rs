use std::collections::HashMap;

pub fn count_duplicates(text: &str) -> u32 {
    let mut map: HashMap<char, u32> = HashMap::new();

    for c in text.to_lowercase().chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    map.values().filter(|&v| *v > 1).count() as u32
}
