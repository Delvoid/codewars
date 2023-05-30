pub fn row_sum_odd_numbers(n: i64) -> i64 {
    let start = n * n - n + 1;
    let end = n * n + n - 1;
    (start..=end).step_by(2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_sum_odd_numbers() {
        assert_eq!(row_sum_odd_numbers(1), 1);
        assert_eq!(row_sum_odd_numbers(2), 8);
        assert_eq!(row_sum_odd_numbers(13), 2197);
        assert_eq!(row_sum_odd_numbers(19), 6859);
        assert_eq!(row_sum_odd_numbers(41), 68921);
    }
}
