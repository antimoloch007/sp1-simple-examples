#![no_std]
#![no_main]

use sp1_zkvm::{entrypoint, io};

// Define the SP1 zkVM entry point
entrypoint!(main);

pub fn main() {
    // Replace this section with any Rust logic you want
    let x: u32 = 10; // Example input 1
    let y: u32 = 20; // Example input 2

    // Perform a computation
    let sum = x + y;

    // Conditional logic
    let result = if sum > 25 {
        sum * 2
    } else {
        sum / 2
    };

    // Commit inputs and results to the proof
    io::commit(&x);
    io::commit(&y);
    io::commit(&sum);
    io::commit(&result);
}
