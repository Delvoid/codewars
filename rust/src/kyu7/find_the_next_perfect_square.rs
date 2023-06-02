pub fn find_next_square(sq: u64) -> Option<u64> {
    let root = (sq as f64).sqrt() as u64;
    if root * root == sq {
        Some((root + 1) * (root + 1))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319225), Some(320356));
        assert_eq!(find_next_square(15241383936), Some(15241630849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342786627), None);
    }
}
