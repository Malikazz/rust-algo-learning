use std::cmp;

#[derive(Default, Debug)]
struct CurrentMax {
    left: usize,
    right: usize,
    area: i32,
    max_possible: i32,
    left_value: i32,
    right_value: i32,
    max_area: i32,
}

impl CurrentMax {

    pub fn calculate_area(&mut self) {
        let min: i32 = cmp::min(self.left_value, self.right_value);
        self.area = min * (self.right - self.left) as i32;
        if self.area > self.max_area {
            self.max_area = self.area;
        }
    }
    pub fn determine_next_move(&mut self) {
        if self.left_value < self.right_value {
            self.left = self.left + 1;
        } else {
            self.right = self.right - 1;
        }
    }

    pub fn calculate_possible_max_left(&mut self, max_height: i32) {
        self.max_possible = (self.right as i32 - self.left as i32) * max_height
    }
}

pub fn solution(height: Vec<i32>) -> i32 {
    const MAX_HEIGHT: i32 = 10_000;
    let mut current_max: CurrentMax = CurrentMax {
        left: 0,
        right: (height.len() - 1),
        area: 0,
        max_possible: 0,
        left_value: height[0],
        right_value: height[height.len() - 1],
        max_area: 0,
    };
    current_max.calculate_area();
    while current_max.left < current_max.right {
        // if the next left and right values are both lower values skip this itteration
        current_max.left_value = height[current_max.left];
        current_max.right_value = height[current_max.right];

        current_max.calculate_area();

        current_max.calculate_possible_max_left(MAX_HEIGHT);
        if current_max.max_area >= current_max.max_possible && current_max.max_possible != 0 {
            return current_max.max_area;
        }

        current_max.determine_next_move();
    }
    current_max.max_area
}

#[cfg(test)]
mod test {
    use crate::container_with_most_water::solution;

    #[test]
    fn test_base_case() {
        let result = solution(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        print!("result: {}\n", result);
        assert!(result == 49)
    }

    #[test]
    fn test_failed_one() {
        let result = solution(vec![2, 3, 10, 5, 7, 8, 9]);
        print!("result: {}\n", result);
        assert!(result == 36)
    }
    #[test]
    fn test_failed_two() {
        let result = solution(vec![1, 0, 0, 0, 0, 0, 0, 2, 2]);
        print!("result: {}\n", result);
        assert!(result == 8)
    }

    #[test]
    fn test_failed_three() {
        let result = solution(vec![1, 8, 6, 2, 5, 4, 8, 25, 7]);
        print!("result: {}\n", result);
        assert!(result == 49)
    }
}
