pub fn divisors(n: u32) -> u32 {
    let mut count = 1;
    let mut prime = 2;
    let mut n = n;

    while prime * prime <= n {
        if n % prime != 0 {
            prime += 1;
        } else {
            let mut prime_count = 0;
            while n % prime == 0 {
                n /= prime;
                prime_count += 1;
            }
            count *= prime_count + 1;
        }
    }

    if n > 1 {
        count *= 2;
    }
    count
}

pub fn divisors_2(n: u32) -> u32 {
    (1..=n).filter(|d| n % d == 0).count() as u32
}
