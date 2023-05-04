pub fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

pub fn repeat_str_2(src: &str, count: usize) -> String {
    let mut result = String::new();
    for _ in 0..count {
        result.push_str(src)
    }

    result
}
