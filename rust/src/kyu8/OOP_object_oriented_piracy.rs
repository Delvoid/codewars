struct Ship {
    draft: u32,
    crew: u32,
}

// Your code here

impl Ship {
    fn new(draft: u32, crew: u32) -> Ship {
        Ship { draft, crew }
    }

    fn is_worth_it(&self) -> bool {
        (self.draft as f64 - self.crew as f64 * 1.5).round() as i32 > 20
    }
}
