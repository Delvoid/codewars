pub fn each_cons(arr: &[u8], n: usize) -> Vec<Vec<u8>> {
    if n == 0 {
        return vec![vec![]];
    }

    if n > arr.len() {
        return vec![];
    }

    let mut result: Vec<Vec<u8>> = Vec::new();
    for i in 0..=arr.len() - n {
        result.push(arr[i..i + n].to_vec());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(
            each_cons(&[1, 2, 3, 4], 2),
            vec![vec![1, 2], vec![2, 3], vec![3, 4]]
        );
        assert_eq!(
            each_cons(&[1, 2, 3, 4], 3),
            vec![vec![1, 2, 3], vec![2, 3, 4]]
        );
        assert_eq!(each_cons(&[1, 2, 3, 4], 4), vec![vec![1, 2, 3, 4]]);
        assert_eq!(each_cons(&[1, 2, 3, 4], 0), vec![vec![]]);
    }
}
