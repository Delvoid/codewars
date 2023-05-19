pub fn tower_builder(n_floors: usize) -> Vec<String> {
    let width = 2 * n_floors - 1;
    let mut tower = Vec::with_capacity(n_floors);

    for floor in 1..=n_floors {
        let stars = 2 * floor - 1;
        let space = (width - stars) / 2;
        let floor_str = format!(
            "{}{}{}",
            " ".repeat(space),
            "*".repeat(stars),
            " ".repeat(space)
        );
        tower.push(floor_str);
    }
    tower
}
