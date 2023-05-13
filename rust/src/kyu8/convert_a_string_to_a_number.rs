pub fn string_to_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

pub fn string_to_number_err(s: &str) -> i32 {
    s.parse::<i32>().expect("Not a number!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(string_to_number("1234"), 1234);
        assert_eq!(string_to_number("605"), 605);
        assert_eq!(string_to_number("1405"), 1405);
        assert_eq!(string_to_number("-7"), -7);
    }
}
