pub fn close_compare(a: f64, b: f64, margin: f64) -> i8 {
    let margin = margin;
    if (a - b).abs() <= margin {
        0
    } else if a < b {
        -1
    } else {
        1
    }
}

pub fn close_compare_1(a: f64, b: f64, margin: Option<f64>) -> i8 {
    let margin = margin.unwrap_or(0.0);

    if (a - b).abs() <= margin {
        0
    } else if a < b {
        -1
    } else {
        1
    }
}
