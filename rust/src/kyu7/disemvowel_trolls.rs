fn disemvowel(s: &str) -> String {
    s.chars().filter(|c| !"aeiouAEIOU".contains(*c)).collect()
}

fn disemvowel2(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {}
            _ => result.push(c),
        }
    }
}
