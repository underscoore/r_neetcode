// Given an integer array nums and an integer k, return the k most frequent elements within the array.
// 
// The test cases are generated such that the answer is always unique.
// 
// You may return the output in any order.
// 
// Example 1:
// 
// Input: nums = [1,2,2,3,3,3], k = 2
// 
// Output: [2,3]
// Example 2:
// 
// Input: nums = [7,7], k = 1
// 
// Output: [7]
// Constraints:
// 
// 1 <= nums.length <= 10^4.
// -1000 <= nums[i] <= 1000
// 1 <= k <= number of distinct elements in nums.

use std::collections::HashMap;

pub fn top_k_frequent_element(numbers: Vec<i32>, k: i32) -> Vec<i32>{
    let mut map = HashMap::new();
    numbers.into_iter()
        .for_each(|number| *map.entry(number).or_insert(0) += 1);
    let mut vec: Vec<(i32, i32)> = map.into_iter().collect();
    vec.sort_by(|a,b| b.1.cmp(&a.1));
    vec.iter().take(k as usize).map(|x| x.0).collect()
}

#[cfg(test)]
mod tests {
    use crate::solutions::p347_solution1::top_k_frequent_element;

    #[test]
    fn unit_top_k_frequent_pos_s1(){
        let numbers: Vec<i32> = vec![1,2,2,3,3,3];
        let k: i32 = 2;
        assert_eq!(top_k_frequent_element(numbers, k),vec![3,2]);
    }

    #[test]
    fn unit_top_k_frequent_two_s1(){
        let numbers: Vec<i32> = vec![7,7];
        let k: i32 = 1;
        assert_eq!(top_k_frequent_element(numbers, k),vec![7]);
    }
}
