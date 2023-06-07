pub fn fake_bin(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.to_digit(10).unwrap() < 5 {
                '0'
            } else {
                '1'
            }
        })
        .collect()
}
