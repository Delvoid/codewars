pub fn round_to_next_5(n: i32) -> i32 {
    let mut n = n;
    while n % 5 != 0 {
        n += 1;
    }
    n
}

fn round_to_next_5_2(n: i32) -> i32 {
    n + (5 - n % 5) % 5
}
