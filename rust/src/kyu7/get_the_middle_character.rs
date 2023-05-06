fn get_middle(s: &str) -> &str {
    let length = s.len();
    let middle = length / 2;

    if length % 2 == 0 {
        &s[middle - 1..middle + 1]
    } else {
        &s[middle..middle + 1]
    }
}
