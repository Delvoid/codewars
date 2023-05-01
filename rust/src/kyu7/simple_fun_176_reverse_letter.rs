pub fn reverse_letters(s: &str) -> String {
    s.chars().rev().filter(|c| c.is_alphabetic()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(reverse_letters("krishan"), "nahsirk");
    }

    #[test]
    fn test2() {
        assert_eq!(reverse_letters("ultr53o?n"), "nortlu");
    }
}
