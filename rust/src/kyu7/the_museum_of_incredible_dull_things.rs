pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut numbers = numbers.to_vec();
    if !numbers.is_empty() {
        let mut smallest = numbers[0];
        let mut index = 0;
        for (i, num) in numbers.iter().enumerate() {
            if num < &smallest {
                smallest = *num;
                index = i
            }
        }
        numbers.remove(index);
    }
    numbers
}
