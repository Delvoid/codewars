pub mod kyu6;
pub mod kyu7;
pub mod kyu8;

pub use kyu6::who_likes_it::likes;
pub use kyu7::reverse_word;
pub use kyu8::count_by_x;
pub use kyu8::difference_of_volumes_of_cuboids;
pub use kyu8::parse_nice_int_from_char_problem;

use crate::parse_nice_int_from_char_problem::get_age;
fn main() {
    println!("Hello, rust codewars!");
    likes(&["delv", "Panda", "creakburn", "rumpo", "athano"]);

    let age = get_age("2 years old");
    println!("{}", age);

    let count_by_x = count_by_x::count_by(50, 5);
    for el in &count_by_x {
        print!("{} ", el);
    }

    let volume_dif = difference_of_volumes_of_cuboids::find_difference(&[19, 24, 7], &[13, 12, 23]);
    println!("volume_dif");
    println!("{}", volume_dif);

    let sentence = "double  spaced  words";
    let reversed = reverse_word::reverse_words(sentence);
    println!("{}", reversed);
}
