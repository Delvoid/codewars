pub fn minimum(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap_or(&0)
}

pub fn maximum(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap_or(&0)
}
