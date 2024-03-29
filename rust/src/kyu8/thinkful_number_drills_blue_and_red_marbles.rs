pub fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
    let blue_left = blue_start - blue_pulled;
    let red_left = red_start - red_pulled;
    let total_left = blue_left + red_left;

    blue_left as f32 / total_left as f32
}

pub fn guess_blue2(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
    (blue_start - blue_pulled) as f32 / (blue_start - blue_pulled + red_start - red_pulled) as f32
}
