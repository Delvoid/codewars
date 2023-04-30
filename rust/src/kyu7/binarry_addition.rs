pub fn add_binary(a: u64, b: u64) -> String {
    let n = a + b;
    format!("{:b}", n)
}
