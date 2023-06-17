pub fn nb_dig(n: i32, d: i32) -> i32 {
    (0..=n)
        .map(|n| n.pow(2).to_string())
        .collect::<String>()
        .chars()
        .filter(|c| c.to_digit(10).unwrap() == d as u32)
        .count() as i32
}

pub fn nb_dig_2(n: i32, d: i32) -> i32 {
    let d = d.to_string();
    (0..=n)
        .map(|n| n.pow(2).to_string().match_indices(&d).count() as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(nb_dig(5750, 0), 4700);
        assert_eq!(nb_dig(11011, 2), 9481);
        assert_eq!(nb_dig(12224, 8), 7733);
        assert_eq!(nb_dig(11549, 1), 11905);
    }

    #[test]
    fn basic_tests_2() {
        assert_eq!(nb_dig_2(5750, 0), 4700);
        assert_eq!(nb_dig_2(11011, 2), 9481);
        assert_eq!(nb_dig_2(12224, 8), 7733);
        assert_eq!(nb_dig_2(11549, 1), 11905);
    }
}
