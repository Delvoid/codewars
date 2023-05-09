use either::Either;

pub fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    arr.iter().fold(0, |acc, x| match x {
        Either::Left(x) => acc + x,
        Either::Right(x) => acc + x.parse::<i32>().unwrap(),
    })
}
