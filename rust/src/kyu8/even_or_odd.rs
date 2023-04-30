pub fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

pub fn even_or_odd_match(i: i32) -> &'static str {
    match i % 2 {
        0 => "Even",
        _ => "Odd",
    }
}

#[cfg(test)]
mod tests {
    use super::even_or_odd;

    #[test]
    fn returns_expected() {
        assert_eq!(even_or_odd(0), "Even");
        assert_eq!(even_or_odd(2), "Even");
        assert_eq!(even_or_odd(1), "Odd");
        assert_eq!(even_or_odd(7), "Odd");
        assert_eq!(even_or_odd(-1), "Odd");
    }
}
