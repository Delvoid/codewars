pub fn break_chocolate(n: u32, m: u32) -> u64 {
    if n == 0 || m == 0 {
        0
    } else {
        (n as u64 * m as u64) - 1
    }
}

pub fn break_chocolate_2(n: u32, m: u32) -> u64 {
    ((n as u64) * (m as u64)).saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(break_chocolate(5, 5), 24);
        assert_eq!(break_chocolate(1, 1), 0);
        assert_eq!(break_chocolate(542803, 308071), 167221863012);
    }
}
