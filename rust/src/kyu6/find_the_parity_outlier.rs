pub fn find_outlier(values: &[i32]) -> i32 {
    let is_even = |x: &i32| x % 2 == 0;
    let mut evens = vec![];
    let mut odds = vec![];

    for i in values {
        if is_even(i) {
            evens.push(*i);
        } else {
            odds.push(*i);
        }

        if evens.len() > 1 && odds.len() == 1 {
            return odds[0];
        } else if odds.len() > 1 && evens.len() == 1 {
            return evens[0];
        }
    }

    panic!("No outlier found")
}
