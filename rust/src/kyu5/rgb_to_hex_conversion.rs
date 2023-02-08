pub fn rgb(r: i32, g: i32, b: i32) -> String {
    let r_hex = decimal_to_hexadecimal(r);
    let g_hex = decimal_to_hexadecimal(g);
    let b_hex = decimal_to_hexadecimal(b);

    format!("#{}{}{}", r_hex, g_hex, b_hex)
}

pub fn decimal_to_hexadecimal(decimal: i32) -> String {
    if decimal <= 0 {
        return "00".to_string();
    }
    if decimal > 255 {
        return "FF".to_string();
    }
    let hex_chars = "0123456789ABCDEF".chars().collect::<Vec<char>>();
    let mut hex = String::new();
    let mut number = decimal;

    while number > 0 {
        hex = hex_chars[(number % 16) as usize].to_string() + &hex;
        number /= 16;
    }

    if hex.len() == 1 {
        format!("0{}", hex)
    } else {
        hex
    }
}

pub fn decimal_to_hex(decimal: i32) -> String {
    if decimal <= 0 {
        return "00".to_string();
    }
    if decimal > 255 {
        return "FF".to_string();
    }
    let hex_chars = "0123456789ABCDEF";
    let mut hex = String::new();
    let mut number = decimal;

    while number > 0 {
        hex = hex_chars
            .chars()
            .nth((number % 16) as usize)
            .unwrap()
            .to_string()
            + &hex;
        number /= 16;
    }

    if hex.len() == 1 {
        format!("0{}", hex)
    } else {
        hex
    }
}
