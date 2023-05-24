pub fn to_camel_case(text: &str) -> String {
    text.split(|c: char| c == '-' || c == '_')
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                s.to_string()
            } else {
                let mut chars = s.chars();
                let first = chars.next().unwrap();
                let rest = chars.as_str();
                format!("{}{}", first.to_uppercase(), rest)
            }
        })
        .collect()
}
