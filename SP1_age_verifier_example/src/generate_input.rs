use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct UserData {
    name: String,
    age: u32,
}

fn main() {
    // Step 1: Read input.json
    let mut file = File::open("input.json").expect("Failed to open input.json");
    let mut json_content = String::new();
    file.read_to_string(&mut json_content).expect("Failed to read input.json");

    // Step 2: Parse JSON and serialize to binary
    let user: UserData = serde_json::from_str(&json_content).expect("Failed to parse JSON");
    let encoded: Vec<u8> = bincode::serialize(&user).expect("Failed to serialize");

    // Step 3: Write binary input to input.bin
    let mut output_file = File::create("input.bin").expect("Failed to create input.bin");
    output_file.write_all(&encoded).expect("Failed to write binary data");

    println!("Binary input written to input.bin");
}
