pub fn min_max(lst: &[i32]) -> (i32, i32) {
    let min = *lst.iter().min().unwrap();
    let max = *lst.iter().max().unwrap();

    (min, max)
}

pub fn min_max_other(lst: &[i32]) -> (i32, i32) {
    lst.iter().fold((i32::MAX, i32::MIN), |(min, max), &x| {
        (min.min(x), max.max(x))
    })
}
