fn triangular(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    (n * (n + 1)) / 2
}
