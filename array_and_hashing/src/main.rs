fn main() {
    let s1: String = "ajay".to_string();
    let s2: String = "yaja".to_string();

    println!("{}",is_anagram(s1, s2));
}

fn is_anagram(string1: String, string2: String) -> bool{
    let mut char_vec1:Vec<char> = string1.chars().collect();
    let mut char_vec2:Vec<char> = string2.chars().collect();

    if string1.len() != string2.len() {
        return false;
    }

    if char_vec1.len() > 0 && char_vec2.len() > 0{
        char_vec1.sort();
        char_vec2.sort();
        return char_vec1 == char_vec2;
    } else {
        return false;
    }

}
