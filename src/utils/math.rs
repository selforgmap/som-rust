/// Calculate Squared Euclidean Distance
/// 
/// # Arguments
/// 
/// * `first` - Vector 1
/// * `second` - Vector 2
pub fn squared_euclidean_distance(first: &Vec<isize>, second: &Vec<isize>) -> isize {
  return first.iter().zip(second)
    .map(|(a, b)| (a - b).pow(2))
    .fold(0isize, |sum, val| sum + val);
}

/// Calculate Euclidean Distance
/// 
/// # Arguments
/// 
/// * `first` - Location 1
/// * `second` - Location 2
pub fn euclidean_distance(first: (f32, f32), second: (f32, f32)) -> f32 {
  return ((first.0 - second.0).powi(2) + (first.1 - second.1).powi(2)).sqrt();
}


/// Unit tests
#[cfg(test)]
mod math_tests {
  #[test]
  fn test_squared_euclidean_distance() {
    let a: Vec<isize> = vec![2, 3, 4];
    let b: Vec<isize> = vec![6, 5, 2];
    assert_eq!(24, super::squared_euclidean_distance(&a, &b));
  }

  #[test]
  fn test_euclidean_distance() {
    assert_eq!(5f32, super::euclidean_distance((2.5f32, 3f32), (6.5f32, 6f32)))
  }
}