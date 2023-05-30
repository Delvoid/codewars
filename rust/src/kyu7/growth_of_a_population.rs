pub fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut population = p0;
    let mut years = 0;
    while population < p {
        population = (population as f64 * (1.0 + percent / 100.0)) as i32 + aug;
        years += 1;
    }
    years
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb_year() {
        assert_eq!(nb_year(1500, 5.0, 100, 5000), 15);
        assert_eq!(nb_year(1500000, 2.5, 10000, 2000000), 10);
        assert_eq!(nb_year(1500000, 0.25, 1000, 2000000), 94);
        assert_eq!(nb_year(1500000, 0.0, 10000, 2000000), 50);
        assert_eq!(nb_year(1000, 2.0, 50, 1214), 4);
    }
}
