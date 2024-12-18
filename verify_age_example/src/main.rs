#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Serialize, Deserialize};
use sp1_zkvm::io;

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    name: String,
    age: u32,
}

pub fn main() {
    // Read input
    let user = io::read::<UserData>();
    println!("Received input: {:?}", user);

    // Check if age > 21
    let is_over_21 = user.age > 21;
    println!("Committed output (is_over_21): {}", is_over_21);
    io::commit(&is_over_21);

    // Print result
    if is_over_21 {
        println!("Hello, {}! You are over 21!", user.name);
    } else {
        println!("Hello, {}! You are not over 21.", user.name);
    }
}
