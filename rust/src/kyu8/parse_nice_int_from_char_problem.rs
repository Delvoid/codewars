pub fn get_age(age: &str) -> u32 {
    let first_char = age.chars().next().unwrap();
    first_char.to_digit(10).unwrap()
}

// taken from solutions
pub fn get_age_2(age: &str) -> u32 {
    age[..1].parse().unwrap()
}
