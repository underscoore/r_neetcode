//Given an array of strings strs, group all anagrams together into sublists. You may return the output in any order.

//An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.

//Example 1:

//Input: strs = ["act","pots","tops","cat","stop","hat"]

//Output: [["hat"],["act", "cat"],["stop", "pots", "tops"]]
//Example 2:

//Input: strs = ["x"]

//Output: [["x"]]
//Example 3:

//Input: strs = [""]

//Output: [[""]]
//Constraints:

//1 <= strs.length <= 1000.
//0 <= strs[i].length <= 100
//strs[i] is made up of lowercase English letteGiven an array of strings strs, group all anagrams together into sublists. You may return the output in any order.
use std::collections::HashMap;

pub fn anagram_group(strs: Vec<&str>) -> Vec<Vec<String>> {
    let mut result: HashMap<Vec<u8>, Vec<String>> = HashMap::new(); 
    for s in strs {
        let mut key: [u8; 26] = [0; 26];
        for c in s.chars() {
            key[c as usize - 'a' as usize] += 1;
        }
        result.entry(key.to_vec()).or_insert(Vec::new()).push(s.to_string());
    }
    result.into_values().collect()
}

#[cfg(test)]
mod tests{
    use crate::solutions::p049_solution1::anagram_group;

    
    #[test]
    fn unit_anagram_group_pos_s1(){
//        let strs = ["act","pots","tops","cat","stop","hat"];
//        let result = anagram_group(strs.to_vec());
        //  XX If you can tell me how to write unit test for this please feel free to raise PR or
    }
}
