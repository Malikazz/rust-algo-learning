#![feature(test)]

extern crate test;


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_project_euler_1(b: &mut Bencher){
        b.iter(|| project_euler_1());
    }

    #[bench]
    fn bench_project_euler_2(b: &mut Bencher){
        b.iter(|| project_euler_2());
    }

}

fn main(){
    project_euler_1();
    project_euler_2();
    project_euler_3();
}

// fn create_prime_vec(end:i64) -> Vec<i64>{
//     let prime_vec: Vec<i64> = Vec::new();
//     let mut add: bool = false;
//     prime_vec.push(2);
//     for num in 2..end{
//         add = false;
//         for prime in prime_vec{
            
//         }
//     }
//     prime_vec
// }

// Project Euler
fn project_euler_1(){
    println!("Project Euler 1\nhttps://projecteuler.net/problem=1");
    let mut total: i64 = 0;
    for num in 0..1000{
        if num % 5 == 0 {
            total += num;
        }
        else if num % 3 == 0{
            total += num;
        }
    }
    println!("Asnwer: {}", total)

}


fn project_euler_2(){
    println!("Project Euler 2\nhttps://projecteuler.net/problem=2");
    let mut previous: i64 = 1;
    let mut current: i64 = 2;
    let mut new: i64 = 0;
    let mut total: i64 = 0;
    while current < 4_000_000{
        if current % 2 == 0{
            total += current;
        }
        new = current + previous;
        previous = current;
        current = new;
    }
    println!("Asnwer: {}", total)
}

fn project_euler_3(){
    println!("Project Euler 3\nhttps://projecteuler.net/problem=3");
    // Generate a prime list using a sudo seive upto 600_851_475_143

}
