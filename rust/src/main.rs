pub mod kyu6;
pub mod kyu8;

pub use kyu6::who_likes_it::likes;
fn main() {
    println!("Hello, rust codewars!");
    likes(&["delv", "Panda", "creakburn", "rumpo", "athano"]);
}
