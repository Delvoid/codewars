pub fn find_digit(num: i32, nth: i32) -> i32 {
    if nth <= 0 {
        return -1;
    }

    let num_str = num.abs().to_string();
    let num_chars: Vec<char> = num_str.chars().rev().collect();

    if (nth as usize) > num_chars.len() {
        return 0;
    }

    num_chars[(nth - 1) as usize].to_digit(10).unwrap() as i32
}
