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
    // Step 1: Read input data
    let user = io::read::<UserData>();
    println!("Received input: {:?}", user);

    // Step 2: Check if age >= 21
    let is_21_or_older = user.age >= 21;
    println!("Committed output (is_21_or_older): {}", is_21_or_older);
    io::commit(&is_21_or_older);

    // Step 3: Output result
    if is_21_or_older {
        println!("Hello, {}! You are 21 or older!", user.name);
    } else {
        println!("Hello, {}! You are younger than 21.", user.name);
    }
}
