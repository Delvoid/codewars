pub fn largest_five_digit_number(num: &str) -> u32 {
    let mut largest = 0;
    for i in 0..num.len() - 4 {
        let slice = &num[i..i + 5];
        let number = slice.parse::<u32>().unwrap();
        if number > largest {
            largest = number;
        }
    }
    largest
}

pub fn largest_five_digit_number_2(num: &str) -> u32 {
    num.as_bytes()
        .windows(5)
        .map(|digits| digits.iter().fold(0, |acc, d| 10 * acc + (d - b'0') as u32))
        .max()
        .unwrap()
}
