pub fn litres(time: f64) -> i32 {
    time as i32 / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_litres() {
        assert_eq!(litres(2.), 1);
        assert_eq!(litres(1.4), 0);
        assert_eq!(litres(12.3), 6);
        assert_eq!(litres(0.82), 0);
        assert_eq!(litres(11.8), 5);
        assert_eq!(litres(1787.), 893);
        assert_eq!(litres(0.), 0);
    }
}
