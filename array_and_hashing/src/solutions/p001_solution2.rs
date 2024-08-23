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
//Purposed solution
//brute force soltion
use std::collections::HashMap;

pub fn two_integer_sum(numbers: &Vec<u32>, target: &u32) -> Vec<u32> {
    let mut num_to_index = HashMap::new();
    for (i, &num) in numbers.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = num_to_index.get(&complement) {
            return vec![index as u32, i as u32];
        }
        num_to_index.insert(num, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::solutions::p001_solution1::two_integer_sum;

    #[test]
    fn unit_two_integer_sum_pos_s2() {
        let numbers: Vec<u32> = vec![4, 5, 6];
        let target: u32 = 10;
        assert_eq!(two_integer_sum(&numbers, &target), vec![0, 2].into());
    }

    #[test]
    fn unit_two_integer_sum_neg_s2() {
        let numbers: Vec<u32> = vec![4, 5, 6];
        let target: u32 = 10;
        assert_ne!(two_integer_sum(&numbers, &target), vec![0, 1].into());
    }

    #[test]
    fn unit_two_integer_sum_dt2_s2() {
        let numbers: Vec<u32> = vec![5, 5];
        let target: u32 = 10;
        assert_eq!(two_integer_sum(&numbers, &target), vec![0, 1].into());
    }
}
