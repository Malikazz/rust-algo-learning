#![feature(test)]
#![feature(iter_advance_by)]
#![allow(dead_code)]

use std::{collections::HashMap};
use std::{collections::VecDeque};
extern crate test;



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_project_euler_1(b: &mut Bencher) {
        b.iter(|| project_euler_1());
    }

    #[bench]
    fn bench_project_euler_2(b: &mut Bencher) {
        b.iter(|| project_euler_2());
    }

    // #[bench]
    // fn bench_project_euler_8_vec(b: &mut Bencher) {
    //     b.iter(|| project_euler_8_vec());
    // }
    #[bench]
    fn bench_sum_four(b: &mut Bencher) {
        let vec_100:Vec<i32> = vec![-14,-71,-13,-2,10,35,-34,-48,22,-82,36,77,-87,-44,-70,-68,-67,-27,-60,11,-69,8,-74,-46,73,5,72,50,-84,-33,60,49,99,-77,-51,55,17,-41,85,21,20,53,56,-9,-29,-62,75,52,-57,-56,63,70,95,31,-55,-99,65,81,39,-52,-95,96,42,-6,62,61,-79,-98,-30,-31,-59,-76,-12,-18,-1,-32,41,23,29,2,27,-47,-90,-66,69,37,-16,13,71,16,-85,-100,9,-23,-88,32,34,-35,-92,54];
        b.iter(|| four_number_sum(vec_100.clone(), 16));
    }

    #[bench]
    fn bench_lru_cache(b: &mut Bencher) {    
        b.iter(|| lru_cache());
    }
}

fn main() {
    lru_cache();
}

// Project Euler
fn project_euler_1() {
    println!("Project Euler 1\nhttps://projecteuler.net/problem=1");
    let mut total: i64 = 0;
    for num in 0..1000 {
        if num % 5 == 0 {
            total += num;
        } else if num % 3 == 0 {
            total += num;
        }
    }
    println!("Asnwer: {}", total)
}

fn project_euler_2() {
    println!("Project Euler 2\nhttps://projecteuler.net/problem=2");
    let mut previous: i64 = 1;
    let mut current: i64 = 2;
    let mut new: i64;
    let mut total: i64 = 0;
    while current < 4_000_000 {
        if current % 2 == 0 {
            total += current;
        }
        new = current + previous;
        previous = current;
        current = new;
    }
    println!("Asnwer: {}", total)
}

// fn project_euler_3(){
//     println!("Project Euler 3\nhttps://projecteuler.net/problem=3");
//     // What is the largest prime factor of the number 600_851_475_143 ?
//     // Find the larget prime factor
//     // Factor and then test for prime?

//     let mut num: i64 = 600_851_475_143;
//     let mut counter: i64 = 0;
//     while num != 1{
//         if num % 2 == 0 {
//             num = num / 2;
//             counter = counter + 2;
//         }
//         else if num % 3 == 0 {
//             num = num / 3;
//             counter = counter +3;
//         }
//         else if num % 5 == 0 {
//             num = num / 5;
//             counter = counter + 5;
//         }
//         else if num % 7 == 0 {
//             num = num / 7;
//             counter = counter + 7;
//         }
//         else {
//             print!("Ran out of factors")
//         }
//     }
//     print!("sqrt root of 600_851_475_143 is {}", counter)

// }

// fn project_euler_8_vec() {
//     //https://projecteuler.net/problem=8
//     print!("Project Euler 8\n");
//     print!("https://projecteuler.net/problem=8\n");

//     let numbers: Vec<u8> = vec![
//         7, 3, 1, 6, 7, 1, 7, 6, 5, 3, 1, 3, 3, 0, 6, 2, 4, 9, 1, 9, 2, 2, 5, 1, 1, 9, 6, 7, 4, 4,
//         2, 6, 5, 7, 4, 7, 4, 2, 3, 5, 5, 3, 4, 9, 1, 9, 4, 9, 3, 4, 9, 6, 9, 8, 3, 5, 2, 0, 3, 1,
//         2, 7, 7, 4, 5, 0, 6, 3, 2, 6, 2, 3, 9, 5, 7, 8, 3, 1, 8, 0, 1, 6, 9, 8, 4, 8, 0, 1, 8, 6,
//         9, 4, 7, 8, 8, 5, 1, 8, 4, 3, 8, 5, 8, 6, 1, 5, 6, 0, 7, 8, 9, 1, 1, 2, 9, 4, 9, 4, 9, 5,
//         4, 5, 9, 5, 0, 1, 7, 3, 7, 9, 5, 8, 3, 3, 1, 9, 5, 2, 8, 5, 3, 2, 0, 8, 8, 0, 5, 5, 1, 1,
//         1, 2, 5, 4, 0, 6, 9, 8, 7, 4, 7, 1, 5, 8, 5, 2, 3, 8, 6, 3, 0, 5, 0, 7, 1, 5, 6, 9, 3, 2,
//         9, 0, 9, 6, 3, 2, 9, 5, 2, 2, 7, 4, 4, 3, 0, 4, 3, 5, 5, 7, 6, 6, 8, 9, 6, 6, 4, 8, 9, 5,
//         0, 4, 4, 5, 2, 4, 4, 5, 2, 3, 1, 6, 1, 7, 3, 1, 8, 5, 6, 4, 0, 3, 0, 9, 8, 7, 1, 1, 1, 2,
//         1, 7, 2, 2, 3, 8, 3, 1, 1, 3, 6, 2, 2, 2, 9, 8, 9, 3, 4, 2, 3, 3, 8, 0, 3, 0, 8, 1, 3, 5,
//         3, 3, 6, 2, 7, 6, 6, 1, 4, 2, 8, 2, 8, 0, 6, 4, 4, 4, 4, 8, 6, 6, 4, 5, 2, 3, 8, 7, 4, 9,
//         3, 0, 3, 5, 8, 9, 0, 7, 2, 9, 6, 2, 9, 0, 4, 9, 1, 5, 6, 0, 4, 4, 0, 7, 7, 2, 3, 9, 0, 7,
//         1, 3, 8, 1, 0, 5, 1, 5, 8, 5, 9, 3, 0, 7, 9, 6, 0, 8, 6, 6, 7, 0, 1, 7, 2, 4, 2, 7, 1, 2,
//         1, 8, 8, 3, 9, 9, 8, 7, 9, 7, 9, 0, 8, 7, 9, 2, 2, 7, 4, 9, 2, 1, 9, 0, 1, 6, 9, 9, 7, 2,
//         0, 8, 8, 8, 0, 9, 3, 7, 7, 6, 6, 5, 7, 2, 7, 3, 3, 3, 0, 0, 1, 0, 5, 3, 3, 6, 7, 8, 8, 1,
//         2, 2, 0, 2, 3, 5, 4, 2, 1, 8, 0, 9, 7, 5, 1, 2, 5, 4, 5, 4, 0, 5, 9, 4, 7, 5, 2, 2, 4, 3,
//         5, 2, 5, 8, 4, 9, 0, 7, 7, 1, 1, 6, 7, 0, 5, 5, 6, 0, 1, 3, 6, 0, 4, 8, 3, 9, 5, 8, 6, 4,
//         4, 6, 7, 0, 6, 3, 2, 4, 4, 1, 5, 7, 2, 2, 1, 5, 5, 3, 9, 7, 5, 3, 6, 9, 7, 8, 1, 7, 9, 7,
//         7, 8, 4, 6, 1, 7, 4, 0, 6, 4, 9, 5, 5, 1, 4, 9, 2, 9, 0, 8, 6, 2, 5, 6, 9, 3, 2, 1, 9, 7,
//         8, 4, 6, 8, 6, 2, 2, 4, 8, 2, 8, 3, 9, 7, 2, 2, 4, 1, 3, 7, 5, 6, 5, 7, 0, 5, 6, 0, 5, 7,
//         4, 9, 0, 2, 6, 1, 4, 0, 7, 9, 7, 2, 9, 6, 8, 6, 5, 2, 4, 1, 4, 5, 3, 5, 1, 0, 0, 4, 7, 4,
//         8, 2, 1, 6, 6, 3, 7, 0, 4, 8, 4, 4, 0, 3, 1, 9, 9, 8, 9, 0, 0, 0, 8, 8, 9, 5, 2, 4, 3, 4,
//         5, 0, 6, 5, 8, 5, 4, 1, 2, 2, 7, 5, 8, 8, 6, 6, 6, 8, 8, 1, 1, 6, 4, 2, 7, 1, 7, 1, 4, 7,
//         9, 9, 2, 4, 4, 4, 2, 9, 2, 8, 2, 3, 0, 8, 6, 3, 4, 6, 5, 6, 7, 4, 8, 1, 3, 9, 1, 9, 1, 2,
//         3, 1, 6, 2, 8, 2, 4, 5, 8, 6, 1, 7, 8, 6, 6, 4, 5, 8, 3, 5, 9, 1, 2, 4, 5, 6, 6, 5, 2, 9,
//         4, 7, 6, 5, 4, 5, 6, 8, 2, 8, 4, 8, 9, 1, 2, 8, 8, 3, 1, 4, 2, 6, 0, 7, 6, 9, 0, 0, 4, 2,
//         2, 4, 2, 1, 9, 0, 2, 2, 6, 7, 1, 0, 5, 5, 6, 2, 6, 3, 2, 1, 1, 1, 1, 1, 0, 9, 3, 7, 0, 5,
//         4, 4, 2, 1, 7, 5, 0, 6, 9, 4, 1, 6, 5, 8, 9, 6, 0, 4, 0, 8, 0, 7, 1, 9, 8, 4, 0, 3, 8, 5,
//         0, 9, 6, 2, 4, 5, 5, 4, 4, 4, 3, 6, 2, 9, 8, 1, 2, 3, 0, 9, 8, 7, 8, 7, 9, 9, 2, 7, 2, 4,
//         4, 2, 8, 4, 9, 0, 9, 1, 8, 8, 8, 4, 5, 8, 0, 1, 5, 6, 1, 6, 6, 0, 9, 7, 9, 1, 9, 1, 3, 3,
//         8, 7, 5, 4, 9, 9, 2, 0, 0, 5, 2, 4, 0, 6, 3, 6, 8, 9, 9, 1, 2, 5, 6, 0, 7, 1, 7, 6, 0, 6,
//         0, 5, 8, 8, 6, 1, 1, 6, 4, 6, 7, 1, 0, 9, 4, 0, 5, 0, 7, 7, 5, 4, 1, 0, 0, 2, 2, 5, 6, 9,
//         8, 3, 1, 5, 5, 2, 0, 0, 0, 5, 5, 9, 3, 5, 7, 2, 9, 7, 2, 5, 7, 1, 6, 3, 6, 2, 6, 9, 5, 6,
//         1, 8, 8, 2, 6, 7, 0, 4, 2, 8, 2, 5, 2, 4, 8, 3, 6, 0, 0, 8, 2, 3, 2, 5, 7, 5, 3, 0, 4, 2,
//         0, 7, 5, 2, 9, 6, 3, 4, 5, 0,
//     ];
    
//     let offset: usize = 13;
//     let mut temp: u64 = 1;
//     let mut highest: u64 = 1;
//     let mut looking = true;
//     let mut current = 1;
//     let mut window = numbers.windows(13);
//     for num in window.into_iter() {
//         if num[0] == 0 {
//             window.advance_back_by(13);
//         }
//         temp = temp * num[0] as u64;
//         highest = if temp > highest {temp} else {highest}
//     }
//     print!("Asnwer:{}\n", highest);
//     }

fn four_number_sum(mut input_array: Vec<i32>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut found_quads: Vec<Vec<i32>> = Vec::new();
    input_array.sort_unstable();

    let offset_1: usize = input_array.len();
    for i in 0..offset_1 - 3 {
        // Skip duplicates
        if i > 0 && input_array[i] == input_array[i - 1] {
            continue;
        }
        for offset_2 in i + 1..offset_1 - 2 {
            if offset_2 != i + 1 && input_array[offset_2] == input_array[offset_2 - 1] {
                continue;
            }
            let mut offset_3 = offset_2 + 1;
            let mut offset_4 = offset_1 - 1;

            while offset_3 < offset_4 {
                let current_sum = input_array[i] + input_array[offset_2] + input_array[offset_3] + input_array[offset_4];
                if current_sum < target_sum {
                    offset_3 += 1;
                }
                else if current_sum > target_sum {
                    offset_4 -= 1;
                }
                else {
                    found_quads.push(vec![input_array[i], input_array[offset_2], input_array[offset_3], input_array[offset_4]]);
                     offset_3 += 1;
                     offset_4 -= 1;
                     while offset_3 < offset_4 && input_array[offset_3] == input_array[offset_3 - 1] {
                        offset_3 += 1;
                     }
                        
                     while offset_3 < offset_4 && input_array[offset_4] == input_array[offset_4 + 1] {
                        offset_4 -= 1;
                     }
                         
                }
            }
        }
    }
    return found_quads
}

const END_KEY: usize = 100001;
const NO_VALUE: i32 = i32::MAX;

#[derive(Copy, Clone)]
struct Node {
    value: i32,
    previous: usize,
    next: usize,
}

impl Node {
    fn new() -> Self {
        Node {
            value: NO_VALUE,
            next: END_KEY,
            previous: END_KEY
        }
    }
}

struct LRUCache {
    lru_cache: Vec<Node>,
    capacity: i32,
    len: usize,
}


impl LRUCache {
    
    fn new (capacity: i32) -> Self {
        let vec_capacity = capacity +1;
        let mut lru = LRUCache { lru_cache: Vec::with_capacity(vec_capacity as usize), capacity:capacity, len:0};
        for _ in 0..vec_capacity{
            lru.lru_cache.push(Node::new());
        }
        lru
    }
    fn pop_lru(&mut self) {
        
    }

    fn get(&mut self, key: i32) -> i32 {
        let key = key as usize;
        if self.lru_cache[key].value != NO_VALUE {
            self.move_to_front(key)
        } else {-1}
    }
    fn put(&mut self, key: i32, value: i32) {
        self.len += 1;
        let key = key as usize;
        
        if (self.capacity <= self.len as i32) {
            
        } 
        self.move_to_front_update(key, value);
    }        
    }
    fn move_to_front(&mut self, key: usize) -> i32 {
        let mut end_node: Node = self.lru_cache[END_KEY];
        let mut current_node: Node = self.lru_cache[key];
        let mut current_last: Node = self.lru_cache[end_node.previous];
        
        
        current_node.next = END_KEY;
        current_node.previous = end_node.previous;

        end_node.previous = key; 
        current_last.next = key;

        current_node.value
    }

    fn move_to_front_update(&mut self, key: usize, value: i32) {
        let mut end_node: Node = self.lru_cache[END_KEY];
        let mut current_node: Node = self.lru_cache[key];
        let mut current_last: Node = self.lru_cache[end_node.previous];
        
        
        current_node.next = END_KEY;
        current_node.previous = end_node.previous;

        end_node.previous = key; 
        current_last.next = key;

        current_node.value = value;
    }
}

fn lru_cache(){
    let mut output: Vec<i32> = Vec::new();

        struct Point {
            key: i32,
            value: i32
        }
        let mut lru_cache = LRUCache::new(2);

        let puts: Vec<Point> = vec![Point{key:1, value:1},Point{key:2, value:2},Point{key:3, value:3},Point{key:4, value:4}];
        let gets = vec![1,2,1,3,4];
        let order = vec!["put","put","get","put","get","put","get","get","get"];
        let mut put_count = 0;
        let mut get_count = 0;
        for ord in order.iter() {
            if ord == &"put" {
                lru_cache.put(puts[put_count].key, puts[put_count].value);
                put_count += 1;
            } else if ord == &"get" {
                output.push(lru_cache.get(gets[get_count]));
                get_count += 1;
            }
        }
        print!("{:?}",output);
}