fn century(year: u32) -> u32 {
    let century = year / 100;
    let decade = year % 100;
    if decade == 0 {
        century
    } else {
        century + 1
    }
}

fn century_2(year: u32) -> u32 {
    (year + 99) / 100
}
