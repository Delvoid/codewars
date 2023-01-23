pub fn are_you_playing_banjo(name: &str) -> String {
    let first_char = name.to_lowercase().chars().next();
    if first_char.map_or(false, |c| c.eq_ignore_ascii_case(&'r')) {
        return format!("{} plays banjo", name);
    }

    format!("{} does not play banjo", name)
}

pub fn are_you_playing_banjoi_match(name: &str) -> String {
    match name.to_lowercase().starts_with('r') {
        true => format!("{} plays banjo", name),
        false => format!("{} does not play banjo", name),
    }
}
