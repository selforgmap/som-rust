extern crate som;

use som::*;

// Main function
fn main() {
    let som = SOM {
        size                  : (10, 10),
        dimension             : 3,
        grid_type             : GridType::Rectangular,
        learning_rate         : 0.1,
        learning_rate_function: LearningRateFunction::Constant,
        initializing_method   : InitializingMethod::Random,
        neighborhood_function : NeighborhoodFunction::Bubble,
        iterations            : 100
    };

    println!("{}", som.iterations); // Test
}
