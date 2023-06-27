pub fn find_even_index(arr: &[i32]) -> Option<usize> {
    let total_sum: i32 = arr.iter().sum();
    let mut left_sum = 0;

    for (i, &item) in arr.iter().enumerate() {
        if left_sum == (total_sum - left_sum - item) {
            return Some(i);
        }
        left_sum += item;
    }

    None
}
