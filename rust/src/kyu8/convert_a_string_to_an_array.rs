pub fn string_to_array(s: &str) -> Vec<String> {
    s.split(" ").map(|s| s.to_string()).collect()
}
