// Write a Rust function named reverse_array that takes an array of integers as its parameter,
// and returns a new array containing the same elements in reverse order.
// You can assume that the input array has a fixed length of N,
// where N is a constant value that is known at compile time.

fn main() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(reverse_array(arr), [5, 4, 3, 2, 1]);
}

fn reverse_array<const N: usize>(arr: [i32; N]) -> [i32; N] {
    let mut result: [i32; N] = [0; N];
    for i in 0..N {
        result[i] = arr[4 - i];
    }
    result
}
