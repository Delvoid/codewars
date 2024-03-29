pub fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow".to_string(),
        "yellow" => "red".to_string(),
        "red" => "green".to_string(),
        _ => panic!("Invalid traffic light color"),
    }
}
