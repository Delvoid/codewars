pub fn square_digits(num: u64) -> u64 {
    let squared_digits: String = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(2))
        .map(|d| d.to_string())
        .collect();

    squared_digits.parse::<u64>().unwrap()
}

pub fn square_digits_2(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).expect("char isnt digit").pow(2).to_string())
        .collect::<String>()
        .parse()
        .expect("string isnt u64")
}
