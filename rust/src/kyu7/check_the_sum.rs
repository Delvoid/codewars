pub fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a
        .iter()
        .zip(arr_b)
        .map(|(correct, given)| {
            if given == correct {
                4
            } else if given.is_empty() {
                0
            } else {
                -1
            }
        })
        .sum::<i64>()
        .max(0)
}

pub fn check_exam_match(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a
        .iter()
        .zip(arr_b)
        .map(|(&a, &b)| match b {
            "" => 0,
            _ if a == b => 4,
            _ => -1,
        })
        .sum::<i64>()
        .max(0)
}
