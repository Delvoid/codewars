pub fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
    if name == "Zach" {
        18
    } else {
        0
    }
}

pub fn how_many_lightsabers_do_you_own_pattern(name: &str) -> u8 {
    match name {
        "Zach" => 18,
        _ => 0,
    }
}
