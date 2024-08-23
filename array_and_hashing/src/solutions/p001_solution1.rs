//POBLEM LeetCode 001
//Problem Statement
//Given an array of integers nums and an integer target, return the indices i and j such that nums[i] + nums[j] == target and i != j.

//You may assume that every input has exactly one pair of indices i and j that satisfy the condition.

//Return the answer with the smaller index first.

//Example 1:

//Input:
//nums = [3,4,5,6], target = 7

//Output: [0,1]
//Explanation: nums[0] + nums[1] == 7, so we return [0, 1].

//Example 2:

//Input: nums = [4,5,6], target = 10

//Output: [0,2]
//Example 3:

//Input: nums = [5,5], target = 10

//Output: [0,1]
//Constraints:
//2 <= nums.length <= 1000
//-10,000,000 <= nums[i] <= 10,000,000
//-10,000,000 <= target <= 10,000,000

use std::collections::HashMap;

// IF IT IS VECTOR
pub fn two_integer_sum(numbers: &Vec<u32>, target: &u32) -> Option<Vec<u32>> {
    let mut prev_map: HashMap<u32, u32> = HashMap::new();
    for (idx, val) in numbers.iter().enumerate() {
        let diff = target - val;
        if prev_map.contains_key(&diff) {
            return Some(vec![prev_map[&diff], idx as u32].into());
        }
        prev_map.insert(*val, idx as u32);
    }
    None
}

pub fn unwrapped_two_integer_sun(numb: &Vec<u32>, &tar: &u32) -> Vec<u32> {
    let two_integer_sum_option = two_integer_sum(&numb, &tar);
    let result = match two_integer_sum_option {
        Some(vec) => vec,
        None => vec![],
    };
    result
}

#[cfg(test)]
mod tests {
    use crate::solutions::p001_solution1::unwrapped_two_integer_sun;

    #[test]
    fn unit_two_integer_sum_pos_s1() {
        let numbers: Vec<u32> = vec![4, 5, 6];
        let target: u32 = 10;
        assert_eq!(unwrapped_two_integer_sun(&numbers, &target), vec![0, 2]);
    }

    #[test]
    fn unit_two_integer_sum_neg_s1() {
        let numbers: Vec<u32> = vec![4, 5, 6];
        let target: u32 = 10;
        assert_ne!(unwrapped_two_integer_sun(&numbers, &target), vec![0, 1]);
    }

    #[test]
    fn unit_two_integer_sum_dt2_s1() {
        let numbers: Vec<u32> = vec![5, 5];
        let target: u32 = 10;
        assert_eq!(unwrapped_two_integer_sun(&numbers, &target), vec![0, 1]);
    }
}
