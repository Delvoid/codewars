pub fn count_bits(n: i64) -> u32 {
    n.count_ones()
}

pub fn count_bits_2(n: i64) -> u32 {
    format!("{:b}", n).chars().filter(|&c| c == '1').count() as u32
}

pub fn count_bits_3(n: i64) -> u32 {
    let mut count = 0;
    let mut n = n;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count as u32
}
