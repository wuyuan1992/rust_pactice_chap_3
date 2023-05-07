// Write a Rust function named average that takes a slice of floating-point numbers as input,
// and returns the average of the numbers in the slice as a f64 value.
// The function should return None if the input slice is empty.

fn main() {
    let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(average(&numbers), Some(3.0));
}

fn average(slice: &[f64]) -> Option<f64> {
    if slice.is_empty() {
        None
    } else {
        let sum: f64 = slice.iter().sum();
        Some(sum / slice.len() as f64)
    }
}
