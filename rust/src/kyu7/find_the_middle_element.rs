pub fn gimme(input_array: [i32; 3]) -> usize {
    let mut sorted_array = input_array;
    sorted_array.sort();
    input_array
        .iter()
        .position(|&x| x == sorted_array[1])
        .unwrap()
}
