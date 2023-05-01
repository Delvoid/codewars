pub fn reverse_seq(n: u32) -> Vec<u32> {
    (1..=n).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(reverse_seq(5), vec![5, 4, 3, 2, 1]);
    }
}
