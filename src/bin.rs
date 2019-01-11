extern crate som;

use som::*;

// Main function
fn main() {
    let item: Vec<i32> = vec![2,3,4];
    let v: Vec<&Vec<i32>> = vec![&item, &item, &item];

    let som = SOM::new((10, 10));
    som.train(&v, 100);

    println!("{}", v[1][2]);
    println!("{}", som.iterations); // Test
}
