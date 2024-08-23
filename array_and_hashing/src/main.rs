mod solutions;
use solutions::p001_solution1;
use solutions::p001_solution2;
use solutions::p049_solution1;
use solutions::p049_solution2;
use solutions::p217_solution1;
use solutions::p217_solution2;
use solutions::p238_solution1;
use solutions::p238_solution2;
use solutions::p242_solution1;
use solutions::p242_solution2;
use solutions::p242_solution3;
use solutions::p347_solution1;
use solutions::p347_solution2;

fn main() {
    let s1: String = "ajay".to_string();
    let s2: String = "yaja".to_string();
    let numbers1: Vec<u32> = vec![1, 2, 3, 3];
    let numbers2: Vec<u32> = vec![1, 2, 3, 4];
    //P001 input data and target
    let numbers3: Vec<u32> = vec![3, 4, 5, 6];
    let target3: u32 = 7;
    //P049 Input data
    let string_vec: Vec<&str> = vec!["act", "pots", "tops", "cat", "stop", "hat"];
    //let string_vec:Vec<&str> = vec!["ab", "ba", "cd", "dc", "z"];
    let numbers4: Vec<i32> = vec![1, 2, 2, 3, 3, 3];
    let frequency: i32 = 2;

    let p238_array: Vec<i32> = vec![1, 2, 3, 4];

    println!("PROBLEM: 242: Is Anagram");
    println!("--> SOLUTION 1");
    println!("{}", p242_solution1::is_anagram(&s1, &s2));
    println!("--> SOLUTION 2");
    println!("{}", p242_solution2::is_anagram(&s1, &s2));
    println!("--> SOLUTION 3");
    println!("{}", p242_solution3::is_anagram(&s1, &s2));
    println!("PROBLEM: 217: DUPLICATE INTEGER");
    println!("{}", p217_solution1::is_duplicate_integer(&numbers1));
    println!("{}", p217_solution1::is_duplicate_integer(&numbers2));
    println!("--> SOLUTION 2");
    println!("{}", p217_solution2::is_duplicate_integer(&numbers1));
    println!("{}", p217_solution2::is_duplicate_integer(&numbers2));
    println!("PROBLEM: 001: TWO INTEGER SUM");
    println!("--> SOLUTION 1");
    println!(
        "{:?}",
        p001_solution1::unwrapped_two_integer_sun(&numbers3, &target3)
    );
    println!("--> SOLUTION 2");
    println!("{:?}", p001_solution2::two_integer_sum(&numbers3, &target3));
    println!("PROBLEM: 049: ANAGRAM GROUP!");
    println!("--> SOLUTION 1");
    println!("{:?}", p049_solution1::anagram_group(string_vec.clone()));
    println!("--> SOLUTION 2");
    println!("{:?}", p049_solution2::anagram_group(string_vec.clone()));
    println!("PROBLEM: 347: TOP K ELEMENTS IN LIST");
    println!("--> SOLUTION 1");
    println!(
        "{:?}",
        p347_solution1::top_k_frequent_element(numbers4.clone(), frequency)
    );
    println!("--> SOLUTION 2");
    println!("{:?}", p347_solution2::top_k_frequent(numbers4, frequency));
    println!("PROBLEM: 238: PRODUCT OF ARRAY EXCEPT SELF");
    println!("--> SOLUTION 1");
    println!("{:?}", p238_solution1::product_of_array(p238_array.clone()));
    println!("--> SOLUTION 2");
    println!("{:?}", p238_solution2::product_of_array(p238_array.clone()));
}
