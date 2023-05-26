fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool {
    if strng.is_empty() {
        return true;
    }

    for i in 0..strng.len() {
        let rot = format!("{}{}", &strng[i..], &strng[..i]);
        if !arr.contains(&rot.as_str()) {
            return false;
        }
    }

    true
}
