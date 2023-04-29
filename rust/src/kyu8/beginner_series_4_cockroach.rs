pub fn cockroach_speed(s: f64) -> i64 {
    (s * 27.77777777777778).floor() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(cockroach_speed(1.08), 30);
        assert_eq!(cockroach_speed(1.09), 30);
        assert_eq!(cockroach_speed(0.0), 0);
    }
}
