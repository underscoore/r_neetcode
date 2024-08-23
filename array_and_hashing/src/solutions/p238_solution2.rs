pub fn product_of_array(array: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![1; (array.len() as i32).try_into().unwrap()];
    let mut postfix = 0;

    for i in 1..array.len() {
        res[i] = res[i - 1] * array[i - 1];
        postfix = 1;
    }
    for j in (0..array.len() - 1).rev() {
        res[j] *= postfix;
        postfix *= array[j];
    }
    res
}
