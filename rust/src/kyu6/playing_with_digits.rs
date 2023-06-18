pub fn dig_pow(n: i64, p: i32) -> i64 {
    let mut sum = 0i64;
    for (i, digit) in n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i64)
        .enumerate()
    {
        sum += digit.pow((p as usize + i) as u32);
    }
    if sum % n == 0 {
        sum / n
    } else {
        -1
    }
}

pub fn dig_pow_2(n: i64, p: i32) -> i64 {
    let sum = n
        .to_string()
        .chars()
        .map(|c| (c as i64 - 48))
        .enumerate()
        .map(|(i, d)| d.pow((p as usize + i) as u32))
        .sum::<i64>();

    match sum % n {
        0 => sum / n,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dig_pow() {
        assert_eq!(dig_pow(89, 1), 1);
        assert_eq!(dig_pow(92, 1), -1);
        assert_eq!(dig_pow(46288, 3), 51);
        assert_eq!(dig_pow(3456789, 5), 51);
    }
}
