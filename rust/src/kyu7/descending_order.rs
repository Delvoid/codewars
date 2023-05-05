pub fn descending_order(x: u64) -> u64 {
    let mut digits: Vec<u64> = x
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u64)
        .collect();
    digits.sort_by(|a, b| b.cmp(a));
    digits.iter().fold(0, |acc, &d| acc * 10 + d)
}
