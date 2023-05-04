pub fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|&n| n.is_positive()).sum()
}

pub fn positive_sum_for(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &n in slice {
        if n > 0 {
            sum += &n
        }
    }
    sum
}

pub fn positive_sum_fold(slice: &[i32]) -> i32 {
    slice
        .iter()
        .fold(0, |acc, &n| if n > 0 { acc + n } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15)
    }

    #[test]
    fn sum_for() {
        assert_eq!(positive_sum_for(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn sum_fold() {
        assert_eq!(positive_sum_fold(&[1, 2, 3, 4, 5]), 15);
    }
}
