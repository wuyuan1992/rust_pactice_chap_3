// Write a function that takes an array of integers
// and returns a new array containing only the even elements of the original array,
// in the same order.

fn main() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(even_elements(&arr).as_slice(), [2, 4]);
}

fn even_elements(slice: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for &num in slice {
        if num % 2 == 0 {
            result.push(num);
        }
    }

    result
}
