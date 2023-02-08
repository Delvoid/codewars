pub fn add_length(s: &str) -> Vec<String> {
    s.split(' ').map(|c| format!("{} {}", c, c.len())).collect()
}

//"apple ban" --> ["apple 5", "ban 3"]
