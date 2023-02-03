pub fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let volume_a: i32 = a.iter().product();
    let volume_b: i32 = b.iter().product();
    (volume_a - volume_b).abs()
}

// one line
pub fn find_differencei_2(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    (a.iter().product::<i32>() - b.iter().product::<i32>()).abs()
}
