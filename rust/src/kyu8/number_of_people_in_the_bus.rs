pub fn number(bus_stops: &[(i32, i32)]) -> i32 {
    let mut count = 0;
    for (on, off) in bus_stops.iter() {
        count += on - off;
    }
    count
}

pub fn number_2(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |acc, x| acc + x.0 - x.1)
}
