pub fn disemvowel(s: &str) -> String {
    s.chars().filter(|c| !"aeiouAEIOU".contains(*c)).collect()
}

pub fn disemvowel2(s: &str) -> String {
    s.replace(['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'], "")
}
