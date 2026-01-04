// https://leetcode.com/problems/two-sum
#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for index in 0..nums.len() {
            result.push(Self::find_first(&nums, index));

            match Self::find_second(&nums, index + 1, target - nums[result[0] as usize]) {
                Ok(equal_index) => {
                    result.push(equal_index);
                    break;
                },
                Err(_) => {
                    result = vec![];
                },
            }
        }

        result
    }

    fn find_first(nums: &Vec<i32>, start_index: usize) -> i32 {
        for index in 0..nums.len() {
            if index < start_index {
                continue;
            }

            return index as i32;
        }
        
        panic!("Incorrect input");
    }

    fn find_second(nums: &Vec<i32>, start_index: usize, target: i32) -> Result<i32, ()> {
        for (index, &num) in nums.iter().enumerate() {
            if index < start_index {
                continue;
            }

            if num == target {
                return Ok(index as i32);
            }
        }
        
        Err(())
    }
}

#[test]
pub fn two_sum_test() {
    let examples = vec![
        (vec![2, 7, 11, 15], 9),
        (vec![3, 2, 4], 6),
        (vec![3, 3], 6),
        (vec![-3, 4, 3, 90], 0),
        (vec![-18, 12, 3, 0], -6),
    ];

    for (nums, target) in examples {
        let result = crate::two_sum::Solution::two_sum(&nums, target);
        assert_eq!(
            nums[result[0] as usize] + nums[result[1] as usize],
            target
        );
    }
}
