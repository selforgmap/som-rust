/// Calculate Squared Euclidean Distance
/// 
/// # Arguments
/// 
/// * `first` - Vector 1
/// * `second` - Vector 2
pub fn squared_euclidean_distance(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
  return first.iter().zip(second)
    .map(|(a, b)| (a - b).pow(2))
    .fold(0i32, |sum, val| sum + val);
}


/// Unit tests
#[cfg(test)]
mod math_tests {
  #[test]
  fn test_squared_euclidean_distance() {
    let a: Vec<i32> = vec![2, 3, 4];
    let b: Vec<i32> = vec![6, 5, 2];
    assert_eq!(24, super::squared_euclidean_distance(&a, &b));
  }
}