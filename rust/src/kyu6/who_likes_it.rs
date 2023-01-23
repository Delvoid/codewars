pub fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        n => format!("{}, {} and {} others like this", names[0], names[1], n - 2),
    }
}

pub fn likes2(names: &[&str]) -> String {
    match names {
        [] => "no one likes this".to_string(),
        [a] => format!("{a} likes this"),
        [a, b] => format!("{a} and {b} like this"),
        [a, b, c] => format!("{a}, {b} and {c} like this"),
        [a, b, rest @ ..] => format!("{a}, {b} and {} others like this", rest.len() - 2),
    }
}
