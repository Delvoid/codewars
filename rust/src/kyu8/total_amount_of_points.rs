pub fn points(games: &[String]) -> u32 {
    games
        .iter()
        .map(|match_result| {
            let scores: Vec<&str> = match_result.split(':').collect();
            let (x, y) = (
                scores[0].parse::<i32>().unwrap(),
                scores[1].parse::<i32>().unwrap(),
            );

            match (x, y) {
                (x, y) if x > y => 3,
                (x, y) if x < y => 0,
                _ => 1,
            }
        })
        .sum()
}

pub fn points_2(games: &[String]) -> u32 {
    games
        .iter()
        .map(|s| {
            let (l, r) = s.split_once(':').unwrap();
            match l.cmp(r) {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 1,
                std::cmp::Ordering::Greater => 3,
            }
        })
        .sum()
}
