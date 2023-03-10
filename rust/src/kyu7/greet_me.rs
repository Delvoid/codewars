pub fn greet(name: &str) -> String {
    let name = name.to_lowercase();
    let first_letter = name.chars().next().unwrap().to_uppercase().to_string();
    let rest_of_name = name.chars().skip(1).collect::<String>();
    let full_name = first_letter + &rest_of_name;

    format!("Hello {}!", full_name)
}

pub fn greet2(name: &str) -> String {
    format!(
        "Hello {}{}!",
        name[..1].to_uppercase(),
        name[1..].to_lowercase()
    )
}
