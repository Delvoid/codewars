fn simple_multiplication(number: u8) -> u8 {
    if number % 2 == 0 {
        number * 8
    } else {
        number * 9
    }
}

fn simple_multiplication_match(number: u8) -> u8 {
    match number % 2 {
        0 => return number * 8,
        _ => return number * 9,
    }
}
