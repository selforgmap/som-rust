// Grid Type
pub enum GridType {
  Rectangular,
  Hexagonal
}

// Learning rate type
pub enum LearningRateType {
  Constant,
  Linear,
  InverseOfTime,
  PowerSeries
}

// Neighborhood type
pub enum NeighborhoodType {
  Bubble
}

// Initializing Method
pub enum InitializingMethod {
  Random,
  PCA
}