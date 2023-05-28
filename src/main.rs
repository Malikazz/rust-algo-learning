#![feature(test)]
#![feature(iter_advance_by)]

pub mod container_with_most_water;
pub mod regular_expression_matching;
pub mod valid_anagram;
fn main() {
    print!(
        "{:?}\n",
        regular_expression_matching::solution(String::from("abc"), String::from("abc"))
    );
    print!("{:?}\n",
        valid_anagram::is_anagram(String::from("car"), String::from("rat"))
        );
}
