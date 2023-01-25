pub mod kyu6;
pub mod kyu7;
pub mod kyu8;

pub use kyu6::who_likes_it::likes;
pub use kyu8::parse_nice_int_from_char_problem;

use crate::parse_nice_int_from_char_problem::get_age;
fn main() {
    println!("Hello, rust codewars!");
    likes(&["delv", "Panda", "creakburn", "rumpo", "athano"]);

    let age = get_age("2 years old");
    println!("{}", age)
}
