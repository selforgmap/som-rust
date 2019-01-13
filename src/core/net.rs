// Neural network struct
pub struct Net {
  pub nodes: Vec<Vec<i32>>,
  pub size: (u32, u32)
}

impl Net {
  // Create new network
  pub fn new(size: (u32, u32), dimension: u32) -> Net {
    Net {
      nodes: vec![vec![0i32; dimension as usize]; (size.0 * size.1) as usize],
      size: size
    }
  }
}