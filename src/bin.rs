extern crate som;

use som::*;

// Main function
fn main() {
    let item: Vec<isize> = vec![2,3,4];
    let v: Vec<&Vec<isize>> = vec![&item, &item, &item];

    let som = SOM::new((10, 10));
    som.train(&v, 100);

    println!("{}", v[1][2]);
    println!("{}", som.iterations); // Test
}
