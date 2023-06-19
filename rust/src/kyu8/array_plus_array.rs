pub fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
    xs.iter().sum::<i32>() + ys.iter().sum::<i32>()
}

pub fn slice_plus_slice_2(xs: &[i32], ys: &[i32]) -> i32 {
    xs.iter().chain(ys).sum()
}
