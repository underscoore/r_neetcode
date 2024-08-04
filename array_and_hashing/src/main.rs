mod solutions;
use solutions::solution1;
use solutions::solution2;
use solutions::solution3;

fn main() {
    let s1: String = "ajay".to_string();
    let s2: String = "yaja".to_string();

    println!("{}", solution1::is_anagram(&s1, &s2));
    println!("{}", solution2::is_anagram(&s1, &s2));
    println!("{}", solution3::is_anagram(&s1, &s2));
}
