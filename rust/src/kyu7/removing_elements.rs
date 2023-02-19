// ["Keep", "Remove", "Keep", "Remove", "Keep", ...] --> ["Keep", "Keep", "Keep", ...]
pub fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    arr.iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, &x)| x)
        .collect()
}

pub fn remove_every_other_2(arr: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for (i, &x) in arr.iter().enumerate() {
        if i % 2 == 0 {
            result.push(x);
        }
    }
    result
}
// reference to an array of unsigned 8-bit integers as its input, and returns a Vec of unsigned 8-bit integers as its output.

// The function uses the iter() method to create an iterator over the input array.
// It then uses the step_by(2) method to create a new iterator that skips every other element of the input array.
// Finally, it uses the cloned() method to create a new iterator that clones each element of the skipped iterator into a new Vec.

pub fn code_wars_answer(arr: &[u8]) -> Vec<u8> {
    arr.iter().step_by(2).cloned().collect()
}
