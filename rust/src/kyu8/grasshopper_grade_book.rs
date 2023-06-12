pub fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let avg = (s1 + s2 + s3) / 3;
    match avg {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_grade() {
        assert_eq!(get_grade(95, 90, 93), 'A');
        assert_eq!(get_grade(100, 85, 96), 'A');
        assert_eq!(get_grade(92, 93, 94), 'A');
        assert_eq!(get_grade(70, 70, 100), 'B');
        assert_eq!(get_grade(82, 85, 87), 'B');
        assert_eq!(get_grade(84, 79, 85), 'B');
        assert_eq!(get_grade(70, 70, 70), 'C');
        assert_eq!(get_grade(75, 70, 79), 'C');
        assert_eq!(get_grade(60, 82, 76), 'C');
        assert_eq!(get_grade(65, 70, 59), 'D');
        assert_eq!(get_grade(66, 62, 68), 'D');
        assert_eq!(get_grade(58, 62, 70), 'D');
        assert_eq!(get_grade(44, 55, 52), 'F');
        assert_eq!(get_grade(48, 55, 52), 'F');
        assert_eq!(get_grade(58, 59, 60), 'F');
        assert_eq!(get_grade(0, 0, 0), 'F');
    }
}
