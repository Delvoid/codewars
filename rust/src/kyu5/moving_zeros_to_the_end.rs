pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut zeros: Vec<u8> = Vec::new();
    let mut filtered: Vec<u8> = Vec::new();

    for &n in arr {
        if n == 0 {
            zeros.push(n);
        } else {
            filtered.push(n);
        }
    }

    filtered.append(&mut zeros);

    filtered
}

pub fn move_zeros_2(arr: &[u8]) -> Vec<u8> {
    arr.iter()
        .filter(|&&n| n != 0)
        .chain(arr.iter().filter(|&&n| n == 0))
        .take(arr.len())
        .cloned()
        .collect()
}
