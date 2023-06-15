pub fn stray(arr: &[u32]) -> u32 {
    let mut result = 0;
    for i in 0..arr.len() {
        if arr[i] != arr[(i + 1) % arr.len()] && arr[i] != arr[(i + 2) % arr.len()] {
            result = arr[i];
            break;
        }
    }
    result
}

pub fn stray_2(arr: &[u32]) -> u32 {
    arr.iter().fold(0, |acc, el| acc ^ el)
}
