pub fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    let mut sides = vec![a, b, c];
    sides.sort();
    sides[0] + sides[1] > sides[2]
}
