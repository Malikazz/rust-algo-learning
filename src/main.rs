#![feature(test)]
#![feature(iter_advance_by)]

pub mod regular_expression_matching;
pub mod container_with_most_water;
fn main() {
    print!(
        "{:?}\n",
        regular_expression_matching::solution(String::from("abc"), String::from("abc"))
    );
}
