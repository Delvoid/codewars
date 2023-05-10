pub fn no_space(x: String) -> String {
    x.replace(" ", "")
}

pub fn no_space_white(x: String) -> String {
    x.split_whitespace().collect()
}
