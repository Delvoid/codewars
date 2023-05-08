fn xo(string: &'static str) -> bool {
    let mut x_count = 0;
    let mut o_count = 0;
    for c in string.chars() {
        match c {
            'x' | 'X' => x_count += 1,
            'o' | 'O' => o_count += 1,
            _ => (),
        }
    }

    x_count == o_count
}

fn xo_2(s: &str) -> bool {
    let s2 = s.to_lowercase();
    s2.matches("x").count() == s2.matches("o").count()
}
