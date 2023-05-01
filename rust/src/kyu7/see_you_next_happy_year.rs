pub fn next_happy_year(year: u16) -> u16 {
    let mut year = year + 1;
    loop {
        let mut digits = Vec::new();
        let mut y = year;
        while y > 0 {
            digits.push(y % 10);
            y /= 10;
        }
        digits.sort();
        digits.dedup();
        if digits.len() == 4 {
            break;
        }
        year += 1;
    }
    year
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(next_happy_year(1001), 1023);
        assert_eq!(next_happy_year(1123), 1203);
        assert_eq!(next_happy_year(2001), 2013);
        assert_eq!(next_happy_year(2334), 2340);
        assert_eq!(next_happy_year(3331), 3401);
        assert_eq!(next_happy_year(1987), 2013);
        assert_eq!(next_happy_year(5555), 5601);
        assert_eq!(next_happy_year(7712), 7801);
        assert_eq!(next_happy_year(8088), 8091);
        assert_eq!(next_happy_year(8999), 9012);
    }
}
