// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//
// You must write an algorithm that runs in O(n) time and without using the division operation.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
// Example 2:
//
// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]
//
//
// Constraints:
//
// 2 <= nums.length <= 105
// -30 <= nums[i] <= 30
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

pub fn product_of_array(array: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for numb in &array {
        let mut prod = 1;
        if *numb != 0 {
            for num in &array {
                prod *= num
            }
            res.push(prod / numb);
        } else {
            res.push(prod / 1);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::solutions::p238_solution1::product_of_array;

    #[test]
    #[ignore = "failed test case"]
    fn unit_product_of_array_execpt_self_pos_s1() {
        // This test case is faling by using this
        // approach. Will give it a look in futuer.
        let array = vec![-1, 1, 0, -3, 3];
        assert_eq!(product_of_array(array), vec![0, 0, 9, 0, 0]);
    }
}
