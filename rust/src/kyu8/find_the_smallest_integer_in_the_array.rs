pub fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}
pub fn find_smallest_int_for(arr: &[i32]) -> i32 {
    let mut smallest = arr[0];
    for &i in arr {
        if i < smallest {
            smallest = i;
        }
    }
    smallest
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
        assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
    }
}
