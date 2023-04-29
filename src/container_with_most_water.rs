use std::cmp;

#[derive(Default, Debug)]
struct CurrentMax {
    left: usize,
    right: usize,
    area: i32
}

impl CurrentMax{
    pub fn new(left:usize, right:usize, area:i32) -> Self{
        Self{ left, right, area}
    }
    pub fn update_max(&mut self, left:usize, right:usize, area:i32){
        self.left = left;
        self.right = right;
        self.area = area;
    }
}

pub fn solution(height: Vec<i32>) -> i32{
    // start at center
    let mut left:usize = 0;
    let mut right:usize = height.len() - 1;
    let mut current_max:CurrentMax = CurrentMax{left, right, area:0};
    let mut current_area:i32;
    while left  < right {
        current_area = calculate_area(left, right, &height);

        if current_area > current_max.area {
            print!("{:?}\n", current_max);
            current_max.update_max(left, right, current_area);
        }
        
        if height[left] < height[right] {
            left = left + 1;
        } else {
            right  = right - 1;
        }
        
    }
    current_max.area 
}

fn calculate_area(left:usize, right:usize, height: &Vec<i32>) -> i32{
    // Get min of the two heights
    let min:i32 = cmp::min(height[left], height[right]);
    let distance = right - left;
    min * distance as i32
}

#[cfg(test)]
mod test {
    use crate::container_with_most_water::solution;

    #[test]
    fn test_base_case() {
        let result = solution(vec![1,8,6,2,5,4,8,3,7]);
        assert!(result == 49)
    }
    
    #[test]
    fn test_failed_one() {
        let result = solution(vec![2,3,10,5,7,8,9]);
        assert!(result == 36)
    }
    #[test]
    fn test_failed_two() {
        let result = solution(vec![1,0,0,0,0,0,0,2,2]);
        assert!(result == 8)
    }

    #[test]
    fn test_failed_three() {
        let result = solution(vec![1,8,6,2,5,4,8,25,7]);
        assert!(result == 49)
    }
}
