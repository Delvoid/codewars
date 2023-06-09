pub fn rps(p1: &str, p2: &str) -> &'static str {
    match (p1, p2) {
        ("rock", "scissors") => "Player 1 won!",
        ("scissors", "paper") => "Player 1 won!",
        ("paper", "rock") => "Player 1 won!",
        ("rock", "paper") => "Player 2 won!",
        ("scissors", "rock") => "Player 2 won!",
        ("paper", "scissors") => "Player 2 won!",
        _ => "Draw!",
    }
}
