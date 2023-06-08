pub fn count_sheep(n: u32) -> String {
    (1..=n).map(|n| format!("{n} sheep...")).collect()
}
