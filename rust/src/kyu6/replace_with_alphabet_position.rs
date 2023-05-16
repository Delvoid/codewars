pub fn alphabet_position(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_alphabetic() {
            result.push_str(&format!("{} ", c.to_ascii_lowercase() as u8 - 96));
        }
    }
    result.trim().to_string()
}

pub fn alphabet_position_2(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c >= &'a' && c <= &'z')
        .map(|c| (c as u32 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
