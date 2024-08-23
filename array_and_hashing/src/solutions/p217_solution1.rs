// The PROBELM
// Duplicate Integer
/*
Given an integer array nums, return true if any value appears more than once in the array, otherwise return false.

Example 1:

Input: nums = [1, 2, 3, 3]

Output: true
Example 2:

Input: nums = [1, 2, 3, 4]

Output: false
*/

use std::collections::HashMap;

// Purposed solution:
// Forst I all insert all the element in HashMap
// Then check if a value against each key is > 1
// If value is greater then 1 then the value is appearing more then once;

pub fn is_duplicate_integer(numbers: &Vec<u32>) -> bool {
    let mut map = HashMap::new();
    if numbers.len() > 0 {
        for num in numbers {
            *map.entry(num).or_insert(0) += 1;
        }
        for (_key, value) in map {
            if value > 1 {
                return false;
            }
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::solutions::p217_solution1::is_duplicate_integer;

    #[test]
    fn unit_is_duplicate_integer_pos_s1() {
        let numbers: Vec<u32> = vec![1, 2, 3];
        assert_eq!(is_duplicate_integer(&numbers), true);
    }
    #[test]
    fn unit_is_duplicate_integer_neg_s1() {
        let numbers: Vec<u32> = vec![1, 2, 3, 3];
        assert_eq!(is_duplicate_integer(&numbers), false);
    }
    #[test]
    fn unit_is_duplicate_integer_emp_s1() {
        let numbers: Vec<u32> = vec![];
        assert_eq!(is_duplicate_integer(&numbers), false);
    }
}
