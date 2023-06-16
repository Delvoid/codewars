pub fn narcissistic_2(num: u64) -> bool {
    let mut digits = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    let len = digits.len() as u32;
    digits.iter().fold(0, |acc, &x| acc + x.pow(len)) == num
}

pub fn narcissistic(num: u64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len() as u32;
    num_str
        .chars()
        .map(|c| (c.to_digit(10).unwrap() as u64).pow(len))
        .sum::<u64>()
        == num
}
