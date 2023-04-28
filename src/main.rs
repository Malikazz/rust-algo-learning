#![feature(test)]
#![feature(iter_advance_by)]

pub mod regular_expression_matching;

fn main() {
    print!(
        "{:?}\n",
        regular_expression_matching::solution(String::from("abc"), String::from("abc"))
    );
}
