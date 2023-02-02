extern crate test;
use std::{collections::HashSet, hash::Hash};
#[cfg(test)]
mod sample_tests {
    use super::*;
    use test::Bencher;

    //#[bench]
    //fn bench_project_euler_1(b: &mut Bencher) {
    //    b.iter(|| project_euler_1());
    //}

    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 5), (6, 10)]), 8, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(11, 15), (6, 10), (1, 2)]), 9, "{}", ERR_MSG);
    }

    #[test]
    fn overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5), (1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 4), (7, 10), (3, 5)]), 7, "{}", ERR_MSG);
    }

    #[test]
    fn large_intervals() {
        assert_eq!(
            sum_intervals(&[(-1_000_000_000, 1_000_000_000)]),
            2_000_000_000,
            "{}",
            ERR_MSG
        );
        assert_eq!(
            sum_intervals(&[(0, 20), (-100_000_000, 10), (30, 40)]),
            100_000_030,
            "{}",
            ERR_MSG
        );
    }

    #[test]
    fn small_intervals() {
        assert_eq!(
            sum_intervals(&[(91, 92), (-32, 96), (82, 83), (-3, 58), (82, 95), (-37, -5), (-98, -30), (-71, 2), (25, 28), (-14, 69), (77, 85), (18, 94), (8, 41), (92, 94), (28, 36), (-76, 38), (97, 99), (-9, 83), (4, 20), (-46, 22), (-20, 49), (97, 98), (14, 76), (45, 76), (-60, -24), (49, 63), (83, 89), (-72, -2), (-79, 70), (-96, -18)]),
            196,
            "{}",
            ERR_MSG
        );
    }
}

pub fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut answer: u32 = 0;

    // reasign to get rid of ref
    let mut intervals = intervals.to_vec();

    let mut combined_intervals: Vec<(i32, i32)> = Vec::new();

    // sort the intervals
    intervals.sort_by(|a, b| a.cmp(&b));

    combined_intervals.push(intervals[0]);

    //search
    for item in intervals.iter() {
        //is there over lap
        for comb_item in 0..combined_intervals.len(){
            // aleady in list
            if combined_intervals.contains(item) {continue;}
            // is fully contained already
            if item.0 <= combined_intervals[comb_item].1 && item.1 <= combined_intervals[comb_item].1 { continue;} 
            //partial contain
            if item.0 <= combined_intervals[comb_item].1 {
                combined_intervals[comb_item].1 = item.1;
            } else{
                combined_intervals.push(*item)
            }   
        }
        print!("{:?}\n", combined_intervals);
    }

    for item in combined_intervals.iter(){
        answer = answer + item.0.abs_diff(item.1)
    }

    answer as i32
}
