pub fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next().unwrap() == dish.chars().next().unwrap()
        && beast.chars().last().unwrap() == dish.chars().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert!(feast("great blue heron", "garlic naan"));
        assert!(feast("chickadee", "chocolate cake"));
        assert!(!feast("brown bear", "bear claw"));
    }
}
