pub fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..=end)
        .filter(|n| !n.to_string().contains('5'))
        .count() as isize
}
