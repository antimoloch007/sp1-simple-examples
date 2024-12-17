#![no_std]
#![no_main]

use sp1_zkvm::{entrypoint, io};

// Define the SP1 zkVM-compatible entry point
entrypoint!(main);

pub fn main() {
    // Input numbers
    let a: u32 = 2;
    let b: u32 = 3;

    // Compute the sum
    let sum = a + b;

    // Commit the result and inputs explicitly
    io::commit(&sum); // Commit the sum
    io::commit(&a);   // Commit input a
    io::commit(&b);   // Commit input b
}
