pub fn evens_and_odds(n: u64) -> String {
    if n % 2 == 0 {
        format!("{:b}", n)
    } else {
        format!("{:x}", n)
    }
}
