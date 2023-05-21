pub fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut v: Vec<String> = s.split_whitespace().map(|x| x.to_string()).collect();
    v.sort_by_key(|a| a.chars().last().unwrap());
    v
}
