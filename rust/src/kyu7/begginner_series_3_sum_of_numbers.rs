pub fn get_sum(a: i64, b: i64) -> i64 {
    let mut sum = 0;
    if a < b {
        for i in a..=b {
            sum += i;
        }
    } else {
        for i in b..=a {
            sum += i;
        }
    }
    sum
}

pub fn get_sum_2(a: i64, b: i64) -> i64 {
    if a < b { a..=b } else { b..=a }.sum()
}

pub fn get_sum_3(a: i64, b: i64) -> i64 {
    (a.min(b)..=a.max(b)).sum()
}
