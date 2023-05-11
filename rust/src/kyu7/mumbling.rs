pub fn accum(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        result.push(c.to_ascii_uppercase());
        for _ in 0..i {
            result.push(c.to_ascii_lowercase());
        }
        if i != s.len() - 1 {
            result.push('-');
        }
    }
    result
}

pub fn accum_2(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(i).as_str()
        })
        .collect::<Vec<String>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accum() {
        assert_eq!(accum("abcd"), "A-Bb-Ccc-Dddd");
        assert_eq!(accum("RqaEzty"), "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy");
        assert_eq!(accum("cwAt"), "C-Ww-Aaa-Tttt");
    }

    #[test]
    fn test_accum_2() {
        assert_eq!(accum_2("abcd"), "A-Bb-Ccc-Dddd");
        assert_eq!(accum_2("RqaEzty"), "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy");
        assert_eq!(accum_2("cwAt"), "C-Ww-Aaa-Tttt");
    }
}
