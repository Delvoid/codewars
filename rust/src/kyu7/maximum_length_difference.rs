pub fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.is_empty() || a2.is_empty() {
        return -1;
    }
    let mut max = 0;
    for s1 in a1.iter() {
        for s2 in a2.iter() {
            let diff = (s1.len() as i32 - s2.len() as i32).abs();
            if diff > max {
                max = diff;
            }
        }
    }
    max
}
