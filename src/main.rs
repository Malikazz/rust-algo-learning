#![feature(test)]
#![feature(iter_advance_by)]

pub mod sum_intervals;

fn main() {
        print!("{:?}\n", sum_intervals::sum_intervals(&[(11, 15), (6, 10), (1, 2)]));
}