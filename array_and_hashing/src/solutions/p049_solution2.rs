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
    for str in strs {
        let mut string_vector: Vec<u8> = Vec::new();
        for ch in str.chars() {
            string_vector.push(ch as u8 - 'a' as u8);
        }
        string_vector.sort();
        result
            .entry(string_vector)
            .or_insert(Vec::new())
            .push(str.to_string());
    }
    result.into_values().collect()
}

// XX If you know how write unit test for this problem please feel free to raise PR or add comment.
