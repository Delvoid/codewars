use std::fmt::format;

pub fn greet(name: &str, owner: &str) -> String {
    match name == owner {
        true => "Hello boss".to_string(),
        _ => "Hello guest".to_string(),
    }
}

pub fn greet2(name: &str, owner: &str) -> String {
    let v = if name == owner { "boss" } else { "guest" };
    format!("Hello {}", v)
}
