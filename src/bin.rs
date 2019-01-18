extern crate som;

use som::*;

// Main function
fn main() {
    let item: Vec<f32> = vec![2f32,3f32,4f32];
    let item2: Vec<f32> = vec![50f32,50f32,50f32];
    let v: Vec<Vec<f32>> = vec![item, item2];

    let som = SOM::new((4, 4));
    let net = som.train(&v, 100);
    
    println!("{}", net);
}
