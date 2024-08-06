// Here in this sliution I am going to use a single HashMap
// Fist I will enter vales in HashMap: Ex if a letter `a` occuress 2 times then key `a`: 2
// then I will be removing the values from HashMap to the corresponding key

use std::collections::HashMap;

pub fn is_anagram(string1: &String, string2: &String) -> bool {
    let mut map = HashMap::new();
    if string1.len() != string2.len() {
        return false;
    }

    for char in string1.chars() {
        *map.entry(char).or_insert(0) += 1;
    }

    for char in string2.chars() {
        *map.entry(char).or_insert(0) -= 1;
    }
    for (_key, value) in map {
        if value != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn unit_is_anagram_pos_s3() {
        let s1: String = "lowlevel".to_string();
        let s2: String = "levellow".to_string();
        assert_eq!(is_anagram(&s1, &s2), true);
    }
    #[test]
    fn unit_is_anagram_neg_s3() {
        let s1: String = "anagram".to_string();
        let s2: String = "margano".to_string();
        assert_eq!(is_anagram(&s1, &s2), false);
    }

    #[test]
    fn unit_is_anagram_ueq_s3() {
        let s1: String = "anagr".to_string();
        let s2: String = "margana".to_string();
        assert_eq!(is_anagram(&s1, &s2), false);
    }
    #[test]
    fn unit_is_anagram_emp_s3() {
        let s1: String = "".to_string();
        let s2: String = "".to_string();
        assert_eq!(is_anagram(&s1, &s2), true);
    }
}
