pub fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
    let mut xs = xs.to_vec();
    xs.sort();
    xs.reverse();
    xs.truncate(n);
    xs.reverse();
    xs
}

#[cfg(test)]
mod tests {
    use super::largest;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(n: usize, xs: &[i32], expected: Vec<i32>) {
        assert_eq!(
            largest(n, xs),
            expected,
            "{ERR_MSG} with n = {n}, xs = {xs:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(2, &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![9, 10]);
    }
}
