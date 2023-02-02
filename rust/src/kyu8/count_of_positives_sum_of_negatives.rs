pub fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return input;
    }

    input.iter().fold(vec![0, 0], |mut total, &x| {
        match x > 0 {
            true => total[0] += 1,
            _ => total[1] += x,
        }
        total
    })
}

pub fn count_positives_sum_negatives2(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return Vec::new();
    }

    vec![
        input.iter().filter(|&&x| x > 0).count() as i32,
        input.iter().filter(|&&x| x < 0).sum(),
    ]
}
