pub fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for i in 1..=n {
        vec.push(x * i);
    }
    vec
}

// solution from CW
pub fn count_by_2(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|e| x * e).collect()
}
