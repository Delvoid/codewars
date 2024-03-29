pub fn quarter_of(month: u8) -> u8 {
    match month {
        1..=3 => 1,
        4..=6 => 2,
        7..=9 => 3,
        10..=12 => 4,
        _ => panic!("Invalid month"),
    }
}

pub fn quarter_of_other(month: u8) -> u8 {
    (month + 2) / 3
}
