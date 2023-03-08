pub fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for i in a..b + 1 {
        let mut sum: u64 = 0;
        let mut j = 1;
        for c in i.to_string().chars() {
            sum += u64::from(c.to_digit(10).unwrap()).pow(j);
            j += 1;
        }
        if sum == i {
            result.push(i);
        }
    }
    result
}
