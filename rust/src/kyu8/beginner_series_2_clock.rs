pub fn past(h: i32, m: i32, s: i32) -> i32 {
    ((h * 60 * 60) + (m * 60) + s) * 1000
}

pub fn past_2(h: i32, m: i32, s: i32) -> i32 {
    h * 3600000 + m * 60000 + s * 1000
}
