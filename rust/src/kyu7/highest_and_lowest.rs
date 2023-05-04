pub fn high_and_low(numbers: &str) -> String {
    let mut numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    numbers.sort();
    format!("{} {}", numbers[numbers.len() - 1], numbers[0])
}
