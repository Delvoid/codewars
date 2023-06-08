pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let divisors: Vec<u32> = (2..integer).filter(|n| integer % n == 0).collect();
    match divisors.len() {
        0 => Err(format!("{} is prime", integer)),
        _ => Ok(divisors),
    }
}
