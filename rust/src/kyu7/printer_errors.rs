pub fn printer_error(s: &str) -> String {
    let mut errors = 0;
    for c in s.chars() {
        if c > 'm' {
            errors += 1;
        }
    }

    format!("{}/{}", errors, s.len())
}

pub fn printer_error_2(s: &str) -> String {
    format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_error() {
        assert_eq!(
            printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "3/56"
        );
        assert_eq!(printer_error("aaaxbbbbyyhwawiwjjjwwm"), "8/22");
    }

    #[test]
    fn test_printer_error_2() {
        assert_eq!(
            printer_error_2("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "3/56"
        );
        assert_eq!(printer_error_2("aaaxbbbbyyhwawiwjjjwwm"), "8/22");
    }
}
