pub fn is_square(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    let root = (n as f64).sqrt() as i64;
    root * root == n
}

pub fn is_square_2(n: i64) -> bool {
    (n as f64).sqrt().fract() == 0.0
}
