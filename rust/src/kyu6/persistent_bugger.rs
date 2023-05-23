pub fn persistence(num: u64) -> u64 {
    let mut num_copy = num;
    if num_copy < 10 {
        return 0;
    }
    let mut product = 1;
    while num_copy > 0 {
        product *= num_copy % 10;
        num_copy /= 10;
    }
    1 + persistence(product)
}

pub fn persistence_2(num: u64) -> u64 {
    let mut count = 0;
    let mut num = num;
    while num >= 10 {
        count += 1;
        num = num
            .to_string()
            .chars()
            .map(|x| x as u64 - 48)
            .product::<u64>();
    }
    count
}
