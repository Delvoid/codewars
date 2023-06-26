pub fn make_readable(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_readable() {
        assert_eq!(make_readable(0), "00:00:00");
        assert_eq!(make_readable(5), "00:00:05");
        assert_eq!(make_readable(60), "00:01:00");
        assert_eq!(make_readable(86399), "23:59:59");
        assert_eq!(make_readable(359999), "99:59:59");
    }
}
