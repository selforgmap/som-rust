// Grid Type
pub enum GridType {
  Rectangular,
  Hexagonal
}

// Learning rate function
pub enum LearningRateFunction {
  Constant,
  Linear,
  InverseOfTime,
  PowerSeries
}

// Neighborhood function
pub enum NeighborhoodFunction {
  Bubble
}

// Initializing Method
pub enum InitializingMethod {
  Random,
  PCA
}