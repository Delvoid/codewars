pub fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let mut result = 0;
    let length = slice.len();

    for (i, &value) in slice.iter().enumerate() {
        if value == 1 {
            result += 2u32.pow((length - i - 1) as u32)
        }
    }

    result
}

pub fn binary_slice_to_number_2(slice: &[u32]) -> u32 {
    slice.iter().rev().enumerate().map(|(i, b)| b << i).sum()
}
