pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + Clone,
{
    sequence.into_iter().fold(Vec::new(), |mut acc, x| {
        if acc.last() != Some(&x) {
            acc.push(x);
        }
        acc
    })
}

pub fn unique_in_order_2<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + Clone,
{
    let mut vec: Vec<_> = sequence.into_iter().collect();
    vec.dedup();
    vec
}
