// Write a Rust function named count_even that takes a slice of integers as its parameter,
// and returns the number of even integers in the slice.

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    assert_eq!(count_even(&numbers), 2);
}

fn count_even(slice: &[i32]) -> i32 {
    let mut count = 0;
    for num in slice {
        if num % 2 == 0 {
            count += 1;
        }
    }
    count
}
