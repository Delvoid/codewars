pub fn digital_root(n: i64) -> i64 {
    let mut sum = n;
    while sum >= 10 {
        sum = sum
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .sum();
    }

    sum
}

pub fn digital_root_2(n: i64) -> i64 {
    if n < 10 {
        n
    } else {
        let sum = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .sum();

        digital_root(sum)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn digital_root_test() {
        let result = digital_root(16);
        let expected = 7;
        assert_eq!(result, expected, "Expected: {expected}, got: {result}")
    }

    #[test]
    fn test_digital_root_single() {
        let result = digital_root(942);
        let expected = 6;
        assert_eq!(result, expected, "Expected: {expected}, got: {result}")
    }
}
