//In this solution I am going creatgin Vec<char> for both of the input strings
//each element of vector is a char type
//then I am sorting both the char array
//the validating both are same, easy pessy

pub fn is_anagram(string1: &String, string2: &String) -> bool {
    let mut char_vec1: Vec<char> = string1.chars().collect();
    let mut char_vec2: Vec<char> = string2.chars().collect();

    if string1.len() != string2.len() {
        return false;
    }

    if char_vec1.len() > 0 && char_vec2.len() > 0 {
        char_vec1.sort();
        char_vec2.sort();
        //        println!("{:?}", char_vec1);
        //        println!("{:?}", char_vec2);
        return char_vec1 == char_vec2;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn unit_is_anagram_pos_s1() {
        let s1: String = "anagram".to_string();
        let s2: String = "margana".to_string();
        assert_eq!(is_anagram(&s1, &s2), true);
    }
    #[test]
    fn unit_is_anagram_nes_s1() {
        let s1: String = "anagram".to_string();
        let s2: String = "margano".to_string();
        assert_eq!(is_anagram(&s1, &s2), false);
    }

    #[test]
    fn unit_is_anagram_ueq_s1() {
        let s1: String = "anagr".to_string();
        let s2: String = "margana".to_string();
        assert_eq!(is_anagram(&s1, &s2), false);
    }
    #[test]
    fn unit_is_anagram_emp_s1() {
        let s1: String = "".to_string();
        let s2: String = "".to_string();
        assert_eq!(is_anagram(&s1, &s2), false);
    }
}
