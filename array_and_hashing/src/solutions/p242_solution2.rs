// In this solution I have created two Empty HashMaps for each strgin
// then insert every char of string in HashMap
// valudate the data inside the HashMap and retun the boolean result.
use std::collections::HashMap;

pub fn is_anagram(string1: &String, string2: &String) -> bool {
    let mut string1_map = HashMap::new();
    let mut string2_map = HashMap::new();

    if &string1.len() != &string2.len() {
        return false;
    }
    string1
        .chars()
        .for_each(|c| *string1_map.entry(c).or_insert(0) += 1);
    string2
        .chars()
        .for_each(|c| *string2_map.entry(c).or_insert(0) += 1);
    //    println!("{:?}", string1_map);
    //    println!("{:?}", string2_map);
    string1_map == string2_map
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn unit_is_anagram_pos_s2() {
        let s1: String = "lowlevel".to_string();
        let s2: String = "levellow".to_string();
        assert_eq!(is_anagram(&s1, &s2), true);
    }
    #[test]
    fn unit_is_anagram_nes_s2() {
        let s1: String = "anagram".to_string();
        let s2: String = "margano".to_string();
        assert_eq!(is_anagram(&s1, &s2), false);
    }

    #[test]
    fn unit_is_anagram_ueq_s2() {
        let s1: String = "anagr".to_string();
        let s2: String = "margana".to_string();
        assert_eq!(is_anagram(&s1, &s2), false);
    }
    #[test]
    fn unit_is_anagram_emp_s2() {
        let s1: String = "".to_string();
        let s2: String = "".to_string();
        assert_eq!(is_anagram(&s1, &s2), true);
    }
}
