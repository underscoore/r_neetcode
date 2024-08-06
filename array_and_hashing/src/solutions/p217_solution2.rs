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

// Purposed solution:
// First I all insert all the element in HashSet 
// Then check if a value is already present in HashSet
// If value is already present in HashSet then return false other wise true

use std::collections::HashSet;

pub fn is_duplicate_integer(numbers: &Vec<u32>) -> bool{
    let mut set = HashSet::new();
    if numbers.len() > 0 {
        for num in numbers{
            if set.contains(num){
                return false;
            }
            set.insert(num);
        }
        //    println!("{:?}", set);
        return true;
    }
    false
}

#[cfg(test)]
mod tests{
    use crate::solutions::p217_solution1::is_duplicate_integer;


    #[test]
    fn unit_id_duplicate_integer_neg_s2(){
        let numbers: Vec<u32> = vec![1,2,3,3];
        assert_eq!(is_duplicate_integer(&numbers), false);
    }

    #[test]
    fn unit_id_duplicate_integer_pos_s2(){
        let numbers: Vec<u32> = vec![1,2,3,4];
        assert_eq!(is_duplicate_integer(&numbers), true);
    }

    #[test]
    fn unit_id_duplicate_integer_pos_s3(){
        let numbers: Vec<u32> = vec![];
        assert_eq!(is_duplicate_integer(&numbers), false);
    }

}

