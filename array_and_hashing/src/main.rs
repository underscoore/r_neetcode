mod solutions;
use solutions::p242_solution1;
use solutions::p242_solution2;
use solutions::p242_solution3;
use solutions::p217_solution1;
use solutions::p217_solution2;
use solutions::p001_solution1;

fn main() {
    let s1: String = "ajay".to_string();
    let s2: String = "yaja".to_string();
    let numbers1: Vec<u32> = vec![1, 2, 3, 3];
    let numbers2: Vec<u32> = vec![1, 2, 3, 4];
    //P001 input data and target 
    let numbers3: Vec<u32> = vec![3,4,5,6];
    let target3: u32 = 7;

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
    println!("{:?}", p001_solution1::unwrapped_two_integer_sun(&numbers3, &target3));

}
